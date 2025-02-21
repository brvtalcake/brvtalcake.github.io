#!/usr/bin/env python3
import sys
import os
import shutil
import subprocess
import argparse
import re
import ipaddress
import signal
import pty
from select import select
import os
import sys
import tty
import termios
import atexit

TO_REMOVE = ['static/syntax-theme-dark.css', 'static/syntax-theme-light.css']

def register_atexit():
    def _todo():
        global TO_REMOVE
        for elem in TO_REMOVE:
            if os.path.isdir(elem):
                shutil.rmtree(elem)
            elif os.path.isfile(elem):
                os.remove(elem)
            elif os.path.exists(elem):
                try:
                    os.unlink(elem)
                except:
                    pass
        return None
    atexit.register(_todo)
    return None
#def signal_handler(sig, frame):
#    global ZOLA_PID
#    if ZOLA_PID is not None:
#        os.kill(ZOLA_PID, sig)
#    for elem in TO_REMOVE:
#        if os.path.isdir(elem):
#            shutil.rmtree(elem)
#        elif os.path.isfile(elem):
#            os.remove(elem)
#        elif os.path.exists(elem):
#            try:
#                os.unlink(elem)
#            except:
#                pass
#
#    sys.exit(0)

def which(program: str):
    prog = shutil.which(program)
    if prog is None:
        raise FileNotFoundError(f'Could not find {program} in PATH')
    return prog

class RunnerOutput:
    def __init__(self, command: list[str] = []):
        self.command = command
        self.output = None
        self.returncode = None
        self.completed = False
        return None

class Runner:
    def __init__(self, command: list[str], **kwargs):
        self.callbacks = (
            kwargs.get(
                'callback_atstart',
                (globals().get('_get_default_runner_callbacks', lambda: (None, None)))()[0]
            ),
            kwargs.get(
                'callback_atexit',
                (globals().get('_get_default_runner_callbacks', lambda: (None, None)))()[1]
            )
        )
        self.print_output = bool(kwargs.get('print_output', False))
        if not self.print_output:
            self.colorized = False
        else:
            self.colorized = bool(kwargs.get('colorized', False))
        self.result = RunnerOutput(command)
        return None

    def run(self):
        if self.colorized:
            self._run_colorized()
        else:
            self._run_uncolorized()
        assert self.result.completed
        return self.result
    
    def _spawn_with_callback(self, master_read, stdin_read, callback):
        """Copied from pty.spawn"""
        argv = self.result.command

        pid, master_fd = pty.fork()
        if pid == pty.CHILD:
            if callback is not None:
                callback(pid)
            os.execlp(argv[0], *argv)

        try:
            mode = tty.tcgetattr(pty.STDIN_FILENO)
            tty.setraw(pty.STDIN_FILENO)
            restore = True
        except tty.error:    # This is the same as termios.error
            restore = False

        try:
            pty._copy(master_fd, master_read, stdin_read)
        finally:
            if restore:
                tty.tcsetattr(pty.STDIN_FILENO, tty.TCSAFLUSH, mode)

        os.close(master_fd)
        return os.waitpid(pid, 0)[1]

    def _run_colorized(self):
        def _readoutput_callback(fd: int):
            data = os.read(fd, 64)
            decoded = data.decode('utf-8')
            if self.result.output is None:
                self.result.output = decoded
            else:
                assert isinstance(self.result.output, str)
                self.result.output += decoded
            return data
        def _notransmit_callback(fd: int):
            # do not transmit data to invoked program's stdin
            return b''
        def _read_callback(fd: int):
            return os.read(fd, 64)
        
        self.result.returncode = self._spawn_with_callback(_readoutput_callback, _read_callback, self.callbacks[0])
        
        if self.callbacks[1] is not None:
            self.callbacks[1](self.result.returncode)
        
        self.result.completed = True
        return None
    
    def _run_uncolorized(self):
        proc = subprocess.Popen(self.result.command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, encoding='utf-8', universal_newlines=True)
        if self.callbacks[0] is not None:
            self.callbacks[0](proc.pid)
        if self.print_output:
            for out in proc.stdout:
                print(out, end='')
            if self.result.output is None:
                self.result.output = out
            else:
                assert isinstance(self.result.output, str)
                self.result.output += out
            self.result.returncode = proc.wait()
        else:
            self.result.output = proc.communicate()[0]
            self.result.returncode = proc.returncode
        if self.callbacks[1] is not None:
            self.callbacks[1](self.result.returncode)
        self.result.completed = True
        return None

def run_zola(command: str, opts: list[str], **kwargs):
    cmdline = [which('zola'), command]
    cmdline.extend([o.strip() for o in opts])
    runner = Runner(cmdline, **kwargs)
    return runner.run()

def get_zola_default(command: str, opt: str):
    output = run_zola(command, ['--help'])
    gotopt = False
    splitted_output = output.output.split('\n')
    indexs = (0, 0)
    for line in splitted_output:
        if line[6:].startswith(opt):
            indexs = (indexs[0], indexs[0] + 1)
            while indexs[0] < len(splitted_output) and \
                  indexs[1] < len(splitted_output) and \
                  not splitted_output[indexs[1]].strip().startswith('-'):
                indexs = (indexs[0], indexs[1] + 1)
            gotopt = True
            break
        indexs = (indexs[0] + 1, 0)
    if not gotopt:
        print(f'Could not find option {opt} in zola {command} --help')
        sys.exit(1)
    res = re.search(r'\[default: (.*)\]', ' '.join(splitted_output[indexs[0]:indexs[1]]))
    if res is None:
        print(f'Could not find default value for {opt} in zola {command} --help')
        sys.exit(1)
    return res.group(1)

def default_interface() -> str:
    ret = get_zola_default('serve', '--interface')
    assert ipaddress.ip_address(ret)
    return ret

def default_port() -> str:
    return get_zola_default('serve', '--port')

def parse_ip(ip: str | None) -> tuple[str, str]:
    if ip is None:
        return (default_interface(), default_port())
    if ip.count(':') == 1:
        try:
            ipaddress.ip_address(ip.split(':')[0])
            int(ip.split(':')[1])
        except ValueError:
            print(f'Invalid IP address {ip}')
            sys.exit(1)
        return ip.split(':')
    elif ip.count(':') > 1:
        ip_part = ip[:ip.rfind(':')]
        port_part = ip[ip.rfind(':')+1:]
        try:
            ipaddress.ip_address(ip_part)
            int(port_part)
        except ValueError:
            print(f'Invalid IP address {ip}')
            sys.exit(1)
        return (ip_part, port_part)
    else:
        try:
            ipaddress.ip_address(ip)
        except ValueError:
            print(f'Invalid IP address {ip}')
            sys.exit(1)
        return (ip, default_port())

def main() -> int:
    parser = argparse.ArgumentParser(description='Serve a Zola site')
    parser.add_argument('ip', nargs='?', help='IP address to bind to (default: Zola default)')
    args = parser.parse_args()
    ip, port = parse_ip(args.ip)
    run_zola('serve', ['--interface', ip, '--port', port], print_output=True, colorized=True)
    return 0

#ZOLA_PID = None
#def _get_default_runner_callbacks():
#    def _set_zola_pid(pid: int):
#        global ZOLA_PID
#        ZOLA_PID = pid
#        return None
#    def _kill_zola_pid(retcode: int):
#        global ZOLA_PID
#        ZOLA_PID = None
#        return None
#    return (_set_zola_pid, _kill_zola_pid)
if __name__ == '__main__':
    #signal.signal(signal.SIGINT, signal_handler)
    #signal.signal(signal.SIGTERM, signal_handler)
    register_atexit()
    sys.exit(main())

#print(get_zola_default('serve', '--interface'))
#print(get_zola_default('serve', '--port'))