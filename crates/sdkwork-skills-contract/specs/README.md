# SDKWork Skills Domain Contract Specs

This directory declares the component contract for `sdkwork-skills-contract`.
The crate owns the surface-neutral Skills records, enums, operation identifiers,
and permission identifiers shared by the Skills Rust layers.

It does not own HTTP transport, persistence, agent sessions, conversations, or
IM messages. Those boundaries remain in their owning modules.

See [component.spec.json](./component.spec.json) and the canonical
[component standard](../../../sdkwork-specs/COMPONENT_SPEC.md).
