export type SkillsConsoleLocale = "en-US" | "zh-CN";

const enUs = {
  "mine.title": "My Skills",
  "mine.description":
    "Skill packages you create stay active in your workspace. Marketplace publication is managed by administrators.",
  "mine.create": "Create skill package",
  "mine.loading": "Loading your skill packages…",
  "mine.empty.title": "No skill packages yet",
  "mine.empty.description": "Create and upload a skill archive to manage it here.",
  "mine.empty.action": "Create and upload a skill package",
  "mine.edit": "Edit",
  "mine.delete": "Delete",
  "mine.column.name": "Name",
  "mine.column.key": "Skill key",
  "mine.column.status": "Status",
  "mine.column.visibility": "Visibility",
  "mine.column.actions": "Actions",
  "mine.delete.confirmTitle": "Delete skill package?",
  "mine.delete.confirmDescription": "Delete “{name}” from your workspace. This cannot be undone.",
  "dialog.cancel": "Cancel",
  "create.title": "Create Skill Package",
  "create.description":
    "Upload a skill archive (through sdkwork-drive) and create an active package that your workspace can install immediately.",
  "create.submit": "Create Package",
  "create.uploading": "Uploading...",
  "create.upload": "Upload Archive via sdkwork-drive",
  "create.uploadedFile": "Uploaded file: {name}",
  "create.created": "Created skill package {id}.",
  "create.error.selectFile": "Select a skill package archive to upload through sdkwork-drive.",
  "create.error.needArtifact":
    "Upload an artifact through sdkwork-drive before creating the skill package.",
  "create.error.checksum": "The uploaded artifact is missing a valid SHA-256 checksum.",
  "create.default.displayName": "Self-service Sample Skill",
  "create.default.summary": "Skill package uploaded by the workspace user",
  "create.field.skillKey": "Skill key",
  "create.field.code": "Package code",
  "create.field.displayName": "Display name",
  "create.field.version": "Artifact version",
  "create.field.entrypoint": "Entrypoint",
  "create.field.archive": "Skill archive",
  "create.field.artifactRef": "Artifact reference",
  "create.placeholder.skillKey": "skill.<segment>.<segment>",
  "create.placeholder.code": "package-code",
  "create.placeholder.displayName": "Display name",
  "create.placeholder.version": "1.0.0",
  "create.placeholder.entrypoint": "run",
  "create.placeholder.artifactRef": "drive://spaces/.../nodes/...",
  "mine.status.active": "active",
  "mine.status.draft": "draft",
  "mine.status.archived": "archived",
  "mine.visibility.private": "private",
  "mine.visibility.tenant": "tenant",
  "mine.visibility.public": "public",
  "edit.title": "Edit Skill Package",
  "edit.back": "Back to My Skills",
  "edit.loading": "Loading…",
  "edit.save": "Save changes",
  "edit.notFound": "Skill package {id} was not found in your workspace.",
  "edit.field.skillKey": "Skill key",
  "edit.field.displayName": "Display name",
  "edit.field.summary": "Summary",
  "edit.field.description": "Description",
  "edit.field.categories": "Categories",
  "edit.field.tags": "Tags",
  "edit.field.categories.hint": "Comma separated",
  "edit.field.tags.hint": "Comma separated",
  "edit.placeholder.displayName": "Display name",
  "edit.placeholder.summary": "Summary",
  "edit.placeholder.description": "Description",
  "edit.placeholder.categories": "category-a, category-b",
  "edit.placeholder.tags": "tag-a, tag-b",
  "edit.aria.skillKey": "Skill key",
} as const;

const zhCn: Record<keyof typeof enUs, string> = {
  "mine.title": "我的 Skills",
  "mine.description": "你创建的 Skill 包会在当前工作区保持可用。上架到市场由管理员处理。",
  "mine.create": "创建 Skill 包",
  "mine.loading": "正在加载你的 Skill 包…",
  "mine.empty.title": "还没有 Skill 包",
  "mine.empty.description": "创建并上传 Skill 压缩包后，可在此管理。",
  "mine.empty.action": "创建并上传 Skill 包",
  "mine.edit": "编辑",
  "mine.delete": "删除",
  "mine.column.name": "名称",
  "mine.column.key": "Skill 标识",
  "mine.column.status": "状态",
  "mine.column.visibility": "可见性",
  "mine.column.actions": "操作",
  "mine.delete.confirmTitle": "删除 Skill 包？",
  "mine.delete.confirmDescription": "将从工作区删除“{name}”，此操作不可撤销。",
  "dialog.cancel": "取消",
  "create.title": "创建 Skill 包",
  "create.description": "通过 sdkwork-drive 上传 Skill 压缩包，并创建工作区可立即安装的可用包。",
  "create.submit": "创建包",
  "create.uploading": "上传中…",
  "create.upload": "通过 sdkwork-drive 上传压缩包",
  "create.uploadedFile": "已上传文件：{name}",
  "create.created": "已创建 Skill 包 {id}。",
  "create.error.selectFile": "请选择要通过 sdkwork-drive 上传的 Skill 压缩包。",
  "create.error.needArtifact": "创建 Skill 包前，请先通过 sdkwork-drive 上传产物。",
  "create.error.checksum": "上传产物缺少有效的 SHA-256 校验和。",
  "create.default.displayName": "自助示例 Skill",
  "create.default.summary": "由工作区用户上传的 Skill 包",
  "create.field.skillKey": "Skill 标识",
  "create.field.code": "包代码",
  "create.field.displayName": "显示名称",
  "create.field.version": "产物版本",
  "create.field.entrypoint": "入口",
  "create.field.archive": "Skill 压缩包",
  "create.field.artifactRef": "产物引用",
  "create.placeholder.skillKey": "skill.<segment>.<segment>",
  "create.placeholder.code": "包代码",
  "create.placeholder.displayName": "显示名称",
  "create.placeholder.version": "1.0.0",
  "create.placeholder.entrypoint": "run",
  "create.placeholder.artifactRef": "drive://spaces/.../nodes/...",
  "mine.status.active": "可用",
  "mine.status.draft": "草稿",
  "mine.status.archived": "已归档",
  "mine.visibility.private": "私有",
  "mine.visibility.tenant": "租户",
  "mine.visibility.public": "公开",
  "edit.title": "编辑 Skill 包",
  "edit.back": "返回我的 Skills",
  "edit.loading": "加载中…",
  "edit.save": "保存更改",
  "edit.notFound": "工作区中未找到 Skill 包 {id}。",
  "edit.field.skillKey": "Skill 标识",
  "edit.field.displayName": "显示名称",
  "edit.field.summary": "摘要",
  "edit.field.description": "描述",
  "edit.field.categories": "分类",
  "edit.field.tags": "标签",
  "edit.field.categories.hint": "逗号分隔",
  "edit.field.tags.hint": "逗号分隔",
  "edit.placeholder.displayName": "显示名称",
  "edit.placeholder.summary": "摘要",
  "edit.placeholder.description": "描述",
  "edit.placeholder.categories": "分类一, 分类二",
  "edit.placeholder.tags": "标签一, 标签二",
  "edit.aria.skillKey": "Skill 标识",
};

export type SkillsConsoleMessageKey = keyof typeof enUs;

const catalogs: Record<SkillsConsoleLocale, Record<SkillsConsoleMessageKey, string>> = {
  "en-US": enUs,
  "zh-CN": zhCn,
};

export function normalizeSkillsConsoleLocale(locale?: string | null): SkillsConsoleLocale {
  // Host/bootstrap injects locale (I18N_SPEC §7). Do not parse document.lang /
  // navigator here — index.html may still say lang="en" while the app is zh-CN.
  if (!locale) {
    return "en-US";
  }
  const normalized = locale.trim().toLowerCase().replaceAll("_", "-");
  return normalized === "zh-cn" || normalized === "zh" || normalized.startsWith("zh-")
    ? "zh-CN"
    : "en-US";
}

export function formatSkillsStatusLocalized(
  locale: SkillsConsoleLocale,
  value: string,
): string {
  switch (value) {
    case "active":
      return translateSkillsConsole(locale, "mine.status.active");
    case "draft":
      return translateSkillsConsole(locale, "mine.status.draft");
    case "archived":
      return translateSkillsConsole(locale, "mine.status.archived");
    default:
      return value;
  }
}

export function formatSkillsVisibilityLocalized(
  locale: SkillsConsoleLocale,
  value: string,
): string {
  switch (value) {
    case "private":
      return translateSkillsConsole(locale, "mine.visibility.private");
    case "tenant":
      return translateSkillsConsole(locale, "mine.visibility.tenant");
    case "public":
      return translateSkillsConsole(locale, "mine.visibility.public");
    default:
      return value;
  }
}

export function translateSkillsConsole(
  locale: SkillsConsoleLocale,
  key: SkillsConsoleMessageKey,
  values: Record<string, string | number> = {},
): string {
  const template = catalogs[locale][key] ?? catalogs["en-US"][key] ?? String(key);
  return Object.entries(values).reduce(
    (message, [name, value]) => message.replaceAll(`{${name}}`, String(value)),
    template,
  );
}
