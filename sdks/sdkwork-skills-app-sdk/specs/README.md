# SDKWork Skills App SDK Specs

This directory declares `sdkwork-skills-app-sdk`, the generated and composed
consumer family for `sdkwork-skills-app-api`.

Consumers import `@sdkwork/skills-app-sdk`. The composed entrypoint owns the
stable client factory; transport output remains exclusively under
`sdkwork-skills-app-sdk-typescript/generated/server-openapi` and is regenerated
from the owner OpenAPI authority.

The family currently exposes 8 owner operations, including explicit published
artifact selection before installation.
