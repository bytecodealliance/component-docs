// See the README for details on *generation* of the required import
import { calculate } from "./bindings/composed.js";

console.log("Answer (to life) = " + calculate.evalExpression("add", 21, 21));
