export async function resolve(specifier, context, nextResolve) {
  if (specifier === '@sdkwork/utils') {
    return {
      url: new URL('./shims/sdkwork-utils-shim.mjs', import.meta.url).href,
      shortCircuit: true,
    };
  }
  return nextResolve(specifier, context);
}
