hotkeys("ctrl+a", function (event, handler) {
  switch (handler.key) {
    case "ctrl+a":
      event.preventDefault();
      alert("you pressed ctrl+a!");
      break;
    default:
      alert(event);
  }
});
