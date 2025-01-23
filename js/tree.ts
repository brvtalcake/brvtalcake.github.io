
import assert from "assert";
import { TupleType } from "typescript";

enum LineType
{
    VERTICAL,   // "│"
    HORIZONTAL, // "─"
    CROSS,      // "┼"
    T,          // "├"
    L,          // "└"
    END         // "┌"
}

type TreeBackground = {
    type: LineType,
    dimensions: {
        width: number,
        height: number
    },
    buffer: Uint32Array
};

function colorToRGBA(color: string): number
{
    let rgba = 0;
    let raw = color;

    if (raw[0] === "#")
        raw = raw.slice(1);

    if (raw.length === 3 || raw.length === 4)
        raw = raw.split("").map(c => c + c).join("");

    if (raw.length !== 8)
        raw += "ff";

    for (let i = 0; i < 4; i++)
        rgba |= parseInt(raw.slice(i * 2, i * 2 + 2), 16) << (i * 8);

    assert(rgba >= 0 && rgba <= 0xffffffff, "Invalid color format");

    return rgba;
}

function between<T>(x: T, a: T, b: T): boolean
{
    return x >= a && x <= b;
}

function buildBG(
    dimensions: Array<number>, lineWidth: number, // both in pixels
    bgColor: string = "#000000",           // color of the background
    lineColor: string = "#ffffff",         // color of the line
    lineType: LineType = LineType.VERTICAL // the type of line to draw
): TreeBackground
{
    assert(dimensions.length === 2, "Invalid dimensions");
    assert(lineWidth > 0, "Invalid line width");

    var bg = {
        type: lineType,
        dimensions: {
            width: dimensions[0],
            height: dimensions[1]
        },
        buffer: new Uint32Array(dimensions[0] * dimensions[1])
    };

    let bgColorRGBA = colorToRGBA(bgColor);
    let lineColorRGBA = colorToRGBA(lineColor);

    for (let y = 0; y < dimensions[1]; y++)
    {
        for (let x = 0; x < dimensions[0]; x++)
        {
            let i = y * dimensions[0] + x;
            let isLine = false;

            switch (lineType)
            {
                case LineType.VERTICAL:
                    isLine = between(
                        x,
                        Math.floor(dimensions[0] / 2) - lineWidth / 2,
                        Math.floor(dimensions[0] / 2) + lineWidth / 2
                    );
                    break;
                case LineType.HORIZONTAL:
                    isLine = between(
                        y,
                        Math.floor(dimensions[1] / 2) - lineWidth / 2,
                        Math.floor(dimensions[1] / 2) + lineWidth / 2
                    );
                    break;
                case LineType.CROSS:
                    isLine = between(
                        x,
                        Math.floor(dimensions[0] / 2) - lineWidth / 2,
                        Math.floor(dimensions[0] / 2) + lineWidth / 2
                    ) || between(
                        y,
                        Math.floor(dimensions[1] / 2) - lineWidth / 2,
                        Math.floor(dimensions[1] / 2) + lineWidth / 2
                    );
                    break;
                case LineType.T:
                    isLine = between(
                        x,
                        Math.floor(dimensions[0] / 2) - lineWidth / 2,
                        Math.floor(dimensions[0] / 2) + lineWidth / 2
                    ) || (
                        between(
                            y,
                            Math.floor(dimensions[1] / 2) - lineWidth / 2,
                            Math.floor(dimensions[1] / 2) + lineWidth / 2
                        ) && x >= Math.floor(dimensions[0] / 2)
                    );
                    break;
                case LineType.L:
                    isLine = between(
                        y,
                        Math.floor(dimensions[1] / 2) - lineWidth / 2,
                        Math.floor(dimensions[1] / 2) + lineWidth / 2
                    ) && x >= Math.floor(dimensions[0] / 2);
                    isLine = isLine || (
                        between(
                            x,
                            Math.floor(dimensions[0] / 2) - lineWidth / 2,
                            Math.floor(dimensions[0] / 2) + lineWidth / 2
                        ) && y <= Math.floor(dimensions[1] / 2)
                    );
                    break;
                case LineType.END:
                    isLine = between(
                        x,
                        Math.floor(dimensions[0] / 2) - lineWidth / 2,
                        Math.floor(dimensions[0] / 2) + lineWidth / 2
                    ) && y <= Math.floor(dimensions[1] / 2);
                    isLine = isLine || (
                        between(
                            y,
                            Math.floor(dimensions[1] / 2) - lineWidth / 2,
                            Math.floor(dimensions[1] / 2) + lineWidth / 2
                        ) && x <= Math.floor(dimensions[0] / 2)
                    );
                    break;
                default:
                    throw new Error("Invalid line type");
            }

            bg.buffer[i] = isLine ? lineColorRGBA : bgColorRGBA;
        }
    }

    return bg;
}



/**
 * TreeMenu
 * @brief Class to handle a `tree`-like menu (same style as the one of `/usr/bin/tree`)
 * @details More specificaly, it sets the background for the different levels of the tree (i.e. "│", "├", "└", "─"),
 * scaled to properly show continuations of the composing lines, no matter the font-size, zoom, etc.
 * @param {HTMLMenuElement} menu
 */
class TreeMenu
{
    constructor(menu)
    {
        
    }
};