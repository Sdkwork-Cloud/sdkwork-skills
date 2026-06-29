---
name: sdkwork-email
description: Use when a Skills workflow needs outbound email; integrate through SDKWork platform services, not local scripts.
---

# SDKWORK Email (Skills)

The Skills application root does not host email transport. Use the SDKWork platform email capability from the owning application or shared service contract when a Skills feature requires notifications.

Do not add email client scripts or legacy Java API wrappers in this repository. Wire through approved SDK families and runtime config from `sdkwork.app.config.json`.
