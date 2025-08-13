import path from "path";
/**
 * @type {import('svgtofont').SvgToFontOptions}
 */
export default {
  fontName: "testIcons",
  classNamePrefix: "icon-prefix",
  css: true,
  svgicons2svgfont: {
    normalize: true,
    fontHeight: 1000
  },
  addLigatures: true
}
