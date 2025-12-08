import { processAndCompareFiles } from "../napi/index.js";

export default function ({ files, referenceTexts }) {
  return processAndCompareFiles(files, referenceTexts, 1.0, "hybrid");
}
