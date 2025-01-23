"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
var assert_1 = __importDefault(require("assert"));
var LineType;
(function (LineType) {
    LineType[LineType["VERTICAL"] = 0] = "VERTICAL";
    LineType[LineType["HORIZONTAL"] = 1] = "HORIZONTAL";
    LineType[LineType["CROSS"] = 2] = "CROSS";
    LineType[LineType["T"] = 3] = "T";
    LineType[LineType["L"] = 4] = "L";
    LineType[LineType["END"] = 5] = "END"; // "┌"
})(LineType || (LineType = {}));
function colorToRGBA(color) {
    var rgba = 0;
    var raw = color;
    if (raw[0] === "#")
        raw = raw.slice(1);
    if (raw.length === 3 || raw.length === 4)
        raw = raw.split("").map(function (c) { return c + c; }).join("");
    if (raw.length !== 8)
        raw += "ff";
    for (var i = 0; i < 4; i++)
        rgba |= parseInt(raw.slice(i * 2, i * 2 + 2), 16) << (i * 8);
    (0, assert_1.default)(rgba >= 0 && rgba <= 0xffffffff, "Invalid color format");
    return rgba;
}
function buildBG(dimensions, lineWidth, // both in pixels
bgColor, // color of the background
lineColor, // color of the line
lineType // the type of line to draw
) {
    if (bgColor === void 0) { bgColor = "#000000"; }
    if (lineColor === void 0) { lineColor = "#ffffff"; }
    if (lineType === void 0) { lineType = LineType.VERTICAL; }
    (0, assert_1.default)(dimensions.length === 2, "Invalid dimensions");
    (0, assert_1.default)(lineWidth > 0, "Invalid line width");
    var bg = {
        type: lineType,
        dimensions: {
            width: dimensions[0],
            height: dimensions[1]
        },
        buffer: new Uint32Array(dimensions[0] * dimensions[1])
    };
    var bgColorRGBA = colorToRGBA(bgColor);
    var lineColorRGBA = colorToRGBA(lineColor);
    for (var y = 0; y < dimensions[1]; y++) {
        for (var x = 0; x < dimensions[0]; x++) {
            var i = y * dimensions[0] + x;
            var isLine = false;
            switch (lineType) {
                case LineType.VERTICAL:
                    isLine = x === Math.floor(dimensions[0] / 2);
                    break;
                case LineType.HORIZONTAL:
                    isLine = y === Math.floor(dimensions[1] / 2);
                    break;
                case LineType.CROSS:
                    isLine = x === Math.floor(dimensions[0] / 2) || y === Math.floor(dimensions[1] / 2);
                    break;
                case LineType.T:
                    isLine = x === Math.floor(dimensions[0] / 2) && y < Math.floor(dimensions[1] / 2);
                    break;
                case LineType.L:
                    isLine = y === Math.floor(dimensions[1] / 2) && x < Math.floor(dimensions[0] / 2);
                    break;
                case LineType.END:
                    isLine = x < Math.floor(dimensions[0] / 2) && y < Math.floor(dimensions[1] / 2);
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
var TreeMenu = /** @class */ (function () {
    function TreeMenu(menu) {
    }
    return TreeMenu;
}());
;
