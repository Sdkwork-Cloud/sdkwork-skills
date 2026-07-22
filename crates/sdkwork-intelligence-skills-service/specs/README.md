# SDKWork Skills Application Service Specs

This directory declares the component contract for
`sdkwork-intelligence-skills-service`.

The service owns Skills validation and use cases. It defines the repository
port but does not own SQL, HTTP request parsing, authentication, or SDK
transport. Artifact selection and installation authorization are expressed as
explicit use cases without a latest-artifact projection.

See [component.spec.json](./component.spec.json) and the canonical
[component standard](../../../sdkwork-specs/COMPONENT_SPEC.md).
