/**
 * This module is the JS implementation of the `string-reverse` WIT world
 */

/**
 * The JavaScript export below represents the export of the `reverse` interface,
 * which which contains `reverse-string` as its primary exported function.
 */
export const reverse = {
/**
 * This JavaScript will be interpreted by `jco` and turned into a
 * WebAssembly binary with a single export (this `reverse` function).
 */
 reverseString(s) {
    return s.split("")
      .reverse()
      .join("");
  }
};
