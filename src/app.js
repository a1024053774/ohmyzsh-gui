const tauriInvoke = window.__TAURI__?.core?.invoke;
const native = Boolean(tauriInvoke);
function readPreference(key, fallback){try{return localStorage.getItem(`ohmyzsh-gui.${key}`)||fallback}catch{return fallback}}
function savePreference(key, value){try{localStorage.setItem(`ohmyzsh-gui.${key}`,value)}catch{}}
const state = {
  section:'overview', snapshot:null, preview:null, selected:null, catalog:[], search:'', configQuery:'',
  status:native?'Loading .zshrc…':'Browser preview · read-only', error:null, token:'',
  updates:null, updatesLoading:false, sort:'stars', sourceFilter:'all', installFilter:'all',
  recommendedLoaded:false, recommendationsLoading:false, configTheme:null, configPlugins:null, categoryFilter:'all',
  language:readPreference('language','auto'), appearance:readPreference('appearance','system'),
  glass:readPreference('glass','on')!=='off', sidebarCollapsed:readPreference('sidebar','open')==='collapsed'
};
const $=id=>document.getElementById(id);
const copy={en:{
  'nav.overview':'Overview','nav.installed':'Installed','nav.configuration':'Configuration','nav.discover':'Discover','nav.updates':'Updates','nav.activity':'Activity','nav.settings':'Settings',
  'title.overview':'Overview','title.installed':'Installed','title.configuration':'Configuration','title.discover':'Discover','title.updates':'Updates','title.activity':'Activity',
  'subtitle.overview':'Review your shell setup before making a change.','subtitle.installed':'Plugins enabled in .zshrc and custom checkouts.','subtitle.configuration':'Edit recognized .zshrc values, then preview before Apply.','subtitle.discover':'Official Oh My Zsh plugins and GitHub repositories.','subtitle.updates':'Review commit changes before applying them.','subtitle.activity':'Backups and reversible operations from this app.',
  'settings.title':'Settings','settings.language':'Language','settings.appearance':'Appearance','settings.liquidGlass':'Liquid Glass materials','settings.glassHint':'Uses translucent materials on the sidebar and chrome, and falls back to solid surfaces when unavailable.','settings.githubToken':'GitHub token','settings.optional':'optional','settings.tokenPrivate':'Token status is private to this app.','settings.tokenHint':'Without a token, GitHub search still works with public rate limits. The token stays local to this app. Save an empty value to remove it.','settings.checkingToken':'Checking token status…','settings.tokenConfigured':'Token is configured in the OS credential store','settings.tokenNone':'No token configured; anonymous GitHub access is active','settings.tokenUnavailable':'Credential store unavailable; anonymous access is active','settings.preferencesSaved':'Preferences saved',
  'common.cancel':'Cancel','common.apply':'Apply','common.save':'Save','common.auto':'Auto (System)','common.light':'Light','common.dark':'Dark','common.english':'English','common.chinese':'简体中文',
  'discover.source':'Source','discover.allSources':'All sources','discover.official':'Oh My Zsh','discover.github':'GitHub','discover.availability':'Availability','discover.allPlugins':'All plugins','discover.available':'Available','discover.installed':'Installed','discover.category':'Type','discover.allCategories':'All types','discover.sort':'Sort','discover.stars':'Most stars','discover.updated':'Recently updated','discover.name':'Name','discover.loading':'Loading GitHub recommendations…','discover.result':'result','discover.results':'results','discover.metadata':'Official plugins are local. GitHub results include stars, license, and update time.',
  'category.theme':'Theme','category.completion':'Completion','category.highlighting':'Highlighting','category.navigation':'Navigation','category.git':'Git','category.manager':'Manager','category.utility':'Utility',
  'overview.heading':'Overview','overview.lede':'A calm place to understand your shell before changing it.','overview.configuration':'Configuration','overview.enabled':'Enabled plugins','overview.updates':'Updates','overview.updatesUnchecked':'Not checked yet','overview.safety':'Safety','overview.safetyText':'Every change is previewed, backed up, syntax-checked, and checked for stale edits before Apply.','overview.readOnly':'Read-only notice','overview.readOnlyText':'Use the raw editor for shell forms this app cannot safely rewrite.','config.heading':'Configuration','config.recognized':'Recognized settings','config.theme':'Theme','config.plugins':'Plugins','config.pluginSearch':'Filter plugins','config.themeHint':'Installed custom themes plus the current ZSH_THEME.','config.pluginsHint':'Only plugins already enabled or installed into the custom directory.','config.preview':'Preview changes','config.apply':'Apply…','config.raw':'Raw source','config.showRaw':'Show exact .zshrc text','config.zshReady':'zsh syntax validation available','config.zshMissing':'zsh unavailable; Apply disabled','config.unavailable':'Configuration unavailable','config.refresh':'Refresh after the shell environment is ready.','installed.empty':'No plugins to manage','installed.start':'Enable an official plugin or install one from Discover.','installed.githubPlugin':'Installed GitHub plugin','installed.customPlugin':'Installed custom plugin','installed.configuredPlugin':'Named in .zshrc','installed.officialPlugin':'Bundled Oh My Zsh plugin','installed.localCheckout':'Local custom checkout','installed.select':'Select a plugin','installed.details':'Details and safe actions appear here.','common.enabled':'Enabled','common.installed':'Installed','common.official':'Official','common.reviewInstall':'Install…','common.reviewUpdate':'Update…','common.reviewUninstall':'Uninstall…','common.enable':'Enable','common.disable':'Disable','common.noResults':'No results','common.adjustFilters':'Search GitHub or adjust the filters.','common.selectPlugin':'Select a plugin','common.detailsActions':'Description, README, and safe actions appear here.','detail.source':'Source','detail.category':'Type','detail.repository':'Repository','detail.stars':'Stars','detail.license':'License','detail.updated':'Updated','detail.currentSha':'Current SHA','detail.availableSha':'Available SHA','detail.notRecorded':'Not recorded','detail.checkUpdate':'Check before update','detail.bundled':'bundled in ohmyzsh/ohmyzsh','detail.safety':'This app never sources plugin code or install scripts. Apply is always a separate confirmation.','detail.readme':'README','detail.loadReadme':'Load README','detail.loadingReadme':'Loading README…','detail.readmeError':'README could not be loaded; open the repository instead.','detail.github':'GitHub','updates.heading':'Updates','updates.lede':'Review commit changes before applying them.','updates.check':'Check for updates','updates.checking':'Checking GitHub…','updates.compare':'Comparing installed plugin commits with their recorded remotes.','updates.available':'Available updates','updates.none':'You’re up to date','updates.noneText':'No app-managed GitHub plugins need an update, or none have been installed through this app yet.','activity.heading':'Activity','activity.lede':'Backups, quarantine paths, and reversible operations.','activity.commit':'Commit','activity.previousCommit':'Previous commit','activity.backupLabel':'Backup','activity.none':'No activity yet','activity.noneText':'Applied operations will appear here.','activity.undo':'Undo','activity.restoreTitle':'Restore this backup?','activity.restoreText':'The app will restore this backup after checking the current .zshrc.','activity.backups':'Configuration backups','activity.keep':'Keep','activity.delete':'Delete','activity.restore':'Restore','status.preview':'Preview ready — review before Apply','status.applied':'Applied','status.readOnly':'Browser preview is read-only','status.review':'Review the commit and diff before Apply','status.searching':'Searching GitHub for','status.githubChecked':'GitHub checked · remaining','status.saved':'Preferences saved','status.footer':'Changes are previewed before Apply','status.searchPlaceholder':'Search plugins','status.toggleSidebar':'Toggle sidebar','status.refresh':'Refresh configuration','status.loaded':'Loaded','status.opening':'Opening GitHub…','status.newTerminal':'Opened a new terminal to load the change','status.reloadHint':'Open a new terminal window to load the change','status.notOmzPlugin':'This checkout is not an Oh My Zsh plugin, so it cannot be added to plugins=()','status.installFirst':'Install this plugin before enabling it',
  'confirm.title':'Apply change?','status.loading':'Loading .zshrc…','status.officialEnable':'Official plugins are enabled from Configuration or the Enable button.'
},zh:{
  'nav.overview':'概览','nav.installed':'已安装','nav.configuration':'配置','nav.discover':'发现','nav.updates':'更新','nav.activity':'活动','nav.settings':'设置',
  'title.overview':'概览','title.installed':'已安装','title.configuration':'配置','title.discover':'发现','title.updates':'更新','title.activity':'活动',
  'subtitle.overview':'修改前先看清当前的 Shell 配置。','subtitle.installed':'已在 .zshrc 启用的插件，以及自定义目录里的插件。','subtitle.configuration':'只改已识别的 .zshrc 项，预览后再应用。','subtitle.discover':'官方 Oh My Zsh 插件和 GitHub 仓库。','subtitle.updates':'应用更新前先查看提交变更。','subtitle.activity':'查看备份和可恢复操作。',
  'settings.title':'设置','settings.language':'语言','settings.appearance':'外观','settings.liquidGlass':'Liquid Glass 材质','settings.glassHint':'只作用于侧边栏和窗口边框；不支持时自动回退为实色。','settings.githubToken':'GitHub Token','settings.optional':'可选','settings.tokenPrivate':'Token 状态仅对本应用可见。','settings.tokenHint':'不配置 Token 也可以搜索 GitHub，但会受公开接口频率限制。Token 只保存在本机，保存空值即可移除。','settings.checkingToken':'正在检查 Token 状态…','settings.tokenConfigured':'Token 已保存在系统凭据库','settings.tokenNone':'未配置 Token，当前使用匿名 GitHub 访问','settings.tokenUnavailable':'系统凭据库不可用，当前使用匿名访问','settings.preferencesSaved':'设置已保存',
  'common.cancel':'取消','common.apply':'应用','common.save':'保存','common.auto':'跟随系统','common.light':'浅色','common.dark':'深色','common.english':'English','common.chinese':'简体中文',
  'discover.source':'来源','discover.allSources':'全部来源','discover.official':'Oh My Zsh','discover.github':'GitHub','discover.availability':'状态','discover.allPlugins':'全部插件','discover.available':'可安装','discover.installed':'已安装','discover.category':'类型','discover.allCategories':'全部类型','discover.sort':'排序','discover.stars':'星标最多','discover.updated':'最近更新','discover.name':'名称','discover.loading':'正在加载 GitHub 推荐…','discover.result':'个结果','discover.results':'个结果','discover.metadata':'官方插件来自本机 Oh My Zsh；GitHub 结果含星标、许可证和更新时间。',
  'category.theme':'主题','category.completion':'补全','category.highlighting':'高亮','category.navigation':'导航','category.git':'Git','category.manager':'管理器','category.utility':'功能',
  'overview.heading':'概览','overview.lede':'修改前先看清当前的 Shell 配置。','overview.configuration':'配置','overview.enabled':'已启用插件','overview.updates':'更新','overview.updatesUnchecked':'尚未检查','overview.safety':'安全性','overview.safetyText':'每次修改都会先预览、备份并通过语法检查，确认后才会应用。','overview.readOnly':'只读提示','overview.readOnlyText':'当前无法安全改写的 Shell 语法，请使用原始编辑器。','config.heading':'配置','config.recognized':'已识别的设置','config.theme':'主题','config.plugins':'插件','config.pluginSearch':'筛选插件','config.themeHint':'来自已安装的自定义主题，以及当前 .zshrc 里的 ZSH_THEME。','config.pluginsHint':'只显示已启用，或已安装到 custom 目录的插件。','config.preview':'预览修改','config.apply':'应用…','config.raw':'原始内容','config.showRaw':'查看完整 .zshrc 文本','config.zshReady':'可进行 zsh 语法验证','config.zshMissing':'未找到 zsh，无法应用修改','config.unavailable':'配置不可用','config.refresh':'刷新后重新读取 Shell 环境。','installed.empty':'没有可管理的插件','installed.start':'在配置里启用官方插件，或到发现页安装。','installed.githubPlugin':'已安装的 GitHub 插件','installed.customPlugin':'已安装的自定义插件','installed.configuredPlugin':'已写入 .zshrc','installed.officialPlugin':'Oh My Zsh 自带插件','installed.localCheckout':'本地自定义检出','installed.select':'选择一个插件','installed.details':'详情和安全操作会显示在这里。','common.enabled':'已启用','common.installed':'已安装','common.official':'官方','common.reviewInstall':'安装…','common.reviewUpdate':'更新…','common.reviewUninstall':'卸载…','common.enable':'启用','common.disable':'停用','common.noResults':'没有结果','common.adjustFilters':'搜索 GitHub 或调整筛选条件。','common.selectPlugin':'选择一个插件','common.detailsActions':'这里会显示简介、README 和安全操作。','detail.source':'来源','detail.category':'类型','detail.repository':'仓库','detail.stars':'星标','detail.license':'许可证','detail.updated':'更新','detail.currentSha':'当前 SHA','detail.availableSha':'可用 SHA','detail.notRecorded':'未记录','detail.checkUpdate':'检查更新后获取','detail.bundled':'内置于 ohmyzsh/ohmyzsh','detail.safety':'本应用不会读取或执行插件安装脚本。应用前始终需要单独确认。','detail.readme':'README','detail.loadReadme':'加载 README','detail.loadingReadme':'正在加载 README…','detail.readmeError':'README 加载失败，可以直接打开仓库。','detail.github':'GitHub','updates.heading':'更新','updates.lede':'应用更新前先查看提交变更。','updates.check':'检查更新','updates.checking':'正在检查 GitHub…','updates.compare':'正在将已安装插件与记录的远端提交进行比较。','updates.available':'可用更新','updates.none':'已经是最新','updates.noneText':'没有需要更新的 GitHub 插件，或还没有通过本应用安装过插件。','activity.heading':'活动','activity.lede':'查看备份、隔离路径和可恢复操作。','activity.commit':'提交','activity.previousCommit':'上一个提交','activity.backupLabel':'备份','activity.none':'还没有活动记录','activity.noneText':'应用过的操作会显示在这里。','activity.undo':'撤销','activity.restoreTitle':'恢复此备份？','activity.restoreText':'应用会检查当前 .zshrc 后恢复这份备份。','activity.backups':'配置备份','activity.keep':'保留','activity.delete':'删除','activity.restore':'恢复','status.preview':'预览已就绪 — 请在应用前检查','status.applied':'已应用','status.readOnly':'浏览器预览为只读模式','status.review':'请在应用前检查提交和差异','status.searching':'正在搜索 GitHub：','status.githubChecked':'GitHub 检查完成 · 剩余请求：','status.saved':'设置已保存','status.footer':'修改会在应用前预览','status.searchPlaceholder':'搜索插件','status.toggleSidebar':'折叠或展开侧边栏','status.refresh':'刷新配置','status.loaded':'已加载','status.opening':'正在打开 GitHub…','status.newTerminal':'已打开新终端以加载这次修改','status.reloadHint':'请打开一个新终端窗口来加载这次修改','status.notOmzPlugin':'这个仓库不是 Oh My Zsh 插件，不能写入 plugins=()','status.installFirst':'请先安装这个插件，再启用',
  'confirm.title':'应用更改？','status.loading':'正在读取 .zshrc…','status.officialEnable':'官方插件请在配置页启用，或使用启用按钮。'
}};
function locale(){return state.language==='auto'?(navigator.language||'').toLowerCase().startsWith('zh')?'zh':'en':state.language}
function t(key){return copy[locale()][key]||copy.en[key]||key}
const FALLBACK_SUMMARY_KEYS=['installed.githubPlugin','installed.customPlugin','installed.configuredPlugin','installed.officialPlugin'];
function allCopy(keys){return keys.flatMap(k=>[copy.en[k],copy.zh[k]])}
function isFallbackSummary(value){return Boolean(value)&&allCopy(FALLBACK_SUMMARY_KEYS).includes(value)}
function pluginSummary(p){
  if(p?.summary&&!isFallbackSummary(p.summary)) return p.summary;
  if(p?.kind==='github') return t('installed.githubPlugin');
  if(p?.kind==='local') return t('installed.customPlugin');
  if(p?.kind==='configured') return t('installed.configuredPlugin');
  return t('installed.officialPlugin');
}
function pluginSource(p){
  if(p?.kind==='local'||allCopy(['installed.localCheckout']).includes(p?.source)) return t('installed.localCheckout');
  return p?.source||'Oh My Zsh';
}
function historyLabel(value){const map={configuration:locale()==='zh'?'配置':'Configuration',applied:locale()==='zh'?'已应用':'Applied',undo:locale()==='zh'?'撤销':'Undo',restored:locale()==='zh'?'已恢复':'Restored',backup_restore:locale()==='zh'?'备份恢复':'Backup restore',install:locale()==='zh'?'安装':'Install',update:locale()==='zh'?'更新':'Update',remove:locale()==='zh'?'卸载':'Remove'};return map[value]||value}
function translateStatic(){
  document.documentElement.lang=locale()==='zh'?'zh-CN':'en';
  document.querySelectorAll('[data-i18n]').forEach(el=>{el.textContent=t(el.dataset.i18n)});
  document.querySelectorAll('[data-i18n-placeholder]').forEach(el=>{el.placeholder=t(el.dataset.i18nPlaceholder)});
  document.querySelectorAll('[data-i18n-title]').forEach(el=>{el.title=t(el.dataset.i18nTitle)});
  document.querySelectorAll('[data-i18n-aria]').forEach(el=>{el.setAttribute('aria-label',t(el.dataset.i18nAria))});
}
function applyPreferences(){
  const theme=state.appearance==='system'?'':state.appearance;
  document.documentElement.dataset.theme=theme;
  document.documentElement.classList.toggle('glass-enabled',state.glass);
  document.documentElement.classList.toggle('sidebar-collapsed',state.sidebarCollapsed);
  document.documentElement.style.colorScheme=theme||'normal';
  translateStatic();
}
function esc(v){return String(v??'').replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]))}
function starIcon(){return '<svg class="meta-icon" viewBox="0 0 24 24" aria-hidden="true"><path fill="currentColor" d="m12 2.8 2.78 5.63 6.22.9-4.5 4.38 1.06 6.2L12 17l-5.56 2.91 1.06-6.2L3 9.33l6.22-.9L12 2.8Z"/></svg>'}
function starMetric(value){return value?`<span class="metric" title="${t('detail.stars')}">${starIcon()}<span>${Number(value).toLocaleString()}</span></span>`:''}
function dateLocale(){return locale()==='zh'?'zh-CN':'en-US'}
function formatDate(value){
  if(!value) return '—';
  const date=new Date(value);
  if(Number.isNaN(date.getTime())) return String(value);
  return new Intl.DateTimeFormat(dateLocale(),{month:'short',day:'numeric'}).format(date);
}
function formatTimestamp(value){
  const seconds=Number(value);
  if(!Number.isFinite(seconds)||seconds<=0) return String(value??'');
  const date=new Date(seconds*1000);
  if(Number.isNaN(date.getTime())) return String(value);
  return new Intl.DateTimeFormat(dateLocale(),{dateStyle:'medium',timeStyle:'short'}).format(date);
}
function pathName(p){
  const s=String(p||'');
  const i=Math.max(s.lastIndexOf('/'),s.lastIndexOf('\\'));
  return i>=0?s.slice(i+1):s;
}
function repoUrl(p){
  if(p.kind==='official') return `https://github.com/ohmyzsh/ohmyzsh/tree/master/plugins/${encodeURIComponent(p.name)}`;
  if(p.repo&&p.repo.owner&&p.repo.owner!=='local') return `https://github.com/${encodeURIComponent(p.repo.owner)}/${encodeURIComponent(p.repo.name)}`;
  return '';
}
function avatarUrl(p){
  if(p.avatar) return p.avatar;
  if(p.kind==='official') return 'https://avatars.githubusercontent.com/ohmyzsh?s=80';
  if(p.repo?.owner&&p.repo.owner!=='local') return `https://avatars.githubusercontent.com/${encodeURIComponent(p.repo.owner)}?s=80`;
  return '';
}
function pluginIcon(p){
  const letter=esc((p.name||'?').slice(0,1).toUpperCase());
  const avatar=avatarUrl(p);
  return `<span class="plugin-icon"><span class="plugin-fallback" aria-hidden="true">${letter}</span>${avatar?`<img class="plugin-avatar" src="${esc(avatar)}" alt="" loading="lazy" onerror="this.remove()" />`:''}</span>`;
}
function externalRepo(p, compact=false){
  const url=repoUrl(p);
  if(!url) return '';
  const label=compact?'↗':`${t('detail.github')} ↗`;
  return `<a class="${compact?'link-icon':'external-link'}" href="${esc(url)}" data-open-url="${esc(url)}" title="${esc(url)}">${label}</a>`;
}
function samePlugin(a,b){
  return a.name===b.name && (a.repo?.owner||'')===(b.repo?.owner||'') && (a.repo?.name||'')===(b.repo?.name||'');
}
function isCheckout(p){return Boolean(p?.installed && (p.kind==='github'||p.kind==='local'))}
function isInUse(p){return Boolean(p?.enabled || isCheckout(p))}
const PLUGIN_CATEGORIES=['theme','completion','highlighting','navigation','git','manager','utility'];
function pluginCategory(p){
  const name=String(p?.name||'').toLowerCase();
  const repo=String(p?.repo?.name||'').toLowerCase();
  const token=name||repo;
  const known={
    'zsh-autosuggestions':'completion','zsh-autocomplete':'completion','autocomplete':'completion',
    'zsh-syntax-highlighting':'highlighting','fast-syntax-highlighting':'highlighting',
    'powerlevel10k':'theme','powerlevel9k':'theme','spaceship-prompt':'theme','spaceship':'theme','starship':'theme',
    'z.lua':'navigation','z':'navigation','zsh-z':'navigation','autojump':'navigation','zoxide':'navigation','fasd':'navigation','enhancd':'navigation',
    'fzf':'completion','fzf-tab':'completion','fzf-marks':'navigation',
    'zinit':'manager','zplug':'manager','antigen':'manager','antibody':'manager','sheldon':'manager','zap':'manager',
    'git':'git','gitfast':'git','extract':'utility'
  };
  if(known[token]) return known[token];
  if(/^(git|gh|hub)([-_]|$)/.test(token)&&!/theme/.test(token)) return 'git';
  const text=`${name} ${repo} ${p?.summary||''}`.toLowerCase();
  const catalog=/awesome[-_ ]|collection of|curated list|plugins, themes/.test(text);
  const rules=[
    !catalog && ['theme',/theme|prompt|powerlevel|p10k|spaceship|starship|agnoster|robbyrussell|typewritten|pure prompt/],
    ['highlighting',/highlight|syntax.?color|colorize|colourize/],
    ['completion',/autosuggest|autocomplete|completion|completions|fzf-tab|\bfzf\b/],
    ['manager',/zinit|zplug|antigen|antibody|sheldon|plugin manager/],
    ['navigation',/autojump|zoxide|z\.lua|bookmark|directory hop|jump around|enhancd/],
    ['git',/(^|[^a-z])git([^a-z]|$)|lazygit|gitstatus/],
    ['utility',/alias|extract|docker|kubectl|sudo|macos|brew|tmux|\bssh\b|\baws\b/]
  ].filter(Boolean);
  for(const [id,re] of rules){ if(re.test(text)) return id; }
  return 'utility';
}
function categoryLabel(p){return t(`category.${pluginCategory(p)}`)}
function categoryBadge(p){return `<span class="badge type">${esc(categoryLabel(p))}</span>`}
function mergeCatalog(previous){
  for(const next of state.catalog){
    const old=previous.find(p=>samePlugin(p,next));
    if(!old) continue;
    next.readme=old.readme;
    next.readmeError=old.readmeError;
    next.current=next.current||old.current;
    next.available=old.available;
    next.stars=next.stars||old.stars;
    next.avatar=next.avatar||old.avatar;
    if((!next.summary||isFallbackSummary(next.summary))&&old.summary&&!isFallbackSummary(old.summary)) next.summary=old.summary;
    next.license=next.license||old.license;
    next.updated=next.updated||old.updated;
  }
  for(const old of previous){
    if(old.kind==='github'&&!old.installed&&!state.catalog.some(p=>samePlugin(p,old))) state.catalog.push(old);
  }
}
function cleanReadme(text){
  const lines=String(text||'').replace(/<!--[\s\S]*?-->/g,'').split('\n');
  const out=[];
  let skipToc=false;
  for(const line of lines){
    let l=line.replace(/<br\s*\/?>/gi,' ').replace(/&nbsp;/gi,' ').replace(/<[^>]+>/g,'');
    l=l.replace(/\[!?\[[^\]]*\]\([^)]*\)\]\([^)]*\)/g,'')
      .replace(/\[!?\[[^\]]*\]\[[^\]]*\]\]\[[^\]]*\]/g,'')
      .replace(/!\[[^\]]*\]\([^)]*\)/g,'')
      .replace(/!\[[^\]]*\]\[[^\]]*\]/g,'');
    const trimmed=l.trim();
    if(/^\[([^\]]+)\]:\s+\S/.test(trimmed)) continue;
    if(/^#{1,6}\s+(table of contents|toc|目录)\b/i.test(trimmed)){skipToc=true;continue}
    if(skipToc){
      if(/^#{1,6}\s+/.test(trimmed)&&!/table of contents|toc|目录/i.test(trimmed)) skipToc=false;
      else continue;
    }
    if(!trimmed){out.push('');continue}
    if(/^(Before|After):?\s*$/i.test(trimmed)) continue;
    if(/shields\.io|\/badge(\.svg|\?)/.test(trimmed)||/^https?:\/\/\S+$/.test(trimmed)) continue;
    out.push(l.replace(/\s+$/g,''));
  }
  return out.join('\n').replace(/\n{3,}/g,'\n\n').trim();
}
function renderReadme(text){
  const lines=cleanReadme(text).slice(0,16000).split('\n');
  const out=[];
  let list=null;
  let fence=null;
  const flush=()=>{if(list){out.push(`<${list.tag}>${list.items.join('')}</${list.tag}>`);list=null}};
  const inline=s=>{
    let t=esc(s);
    t=t.replace(/\[!?\[[^\]]*\]\([^)]*\)\]\([^)]*\)/g,'');
    t=t.replace(/\[!?\[[^\]]*\]\[[^\]]*\]\]\[[^\]]*\]/g,'');
    t=t.replace(/!\[[^\]]*\]\([^)]*\)/g,'');
    t=t.replace(/!\[[^\]]*\]\[[^\]]*\]/g,'');
    t=t.replace(/\[\]\([^)]*\)/g,'');
    t=t.replace(/`([^`]+)`/g,'<code>$1</code>');
    t=t.replace(/\*\*([^*]+)\*\*/g,'<strong>$1</strong>');
    t=t.replace(/__([^_]+)__/g,'<strong>$1</strong>');
    t=t.replace(/(^|[^\w*])\*([^*\n]+)\*(?=[^\w*]|$)/g,'$1<em>$2</em>');
    t=t.replace(/(^|[^\w_])_([^_\n]+)_(?=[^\w_]|$)/g,'$1<em>$2</em>');
    t=t.replace(/\[([^\]]+)\]\((https?:[^)\s]+)\)/g,'<a href="$2" data-open-url="$2">$1</a>');
    t=t.replace(/\[([^\]]+)\]\[[^\]]*\]/g,'$1');
    t=t.replace(/\[([^\]]+)\]\([^)]+\)/g,'$1');
    return t;
  };
  for(let i=0;i<lines.length;i++){
    const raw=lines[i].trimEnd();
    const trimmed=raw.trim();
    const next=(lines[i+1]||'').trim();
    if(trimmed.startsWith('```')){
      if(fence){out.push(`<pre><code>${esc(fence.join('\n'))}</code></pre>`);fence=null}
      else {flush();fence=[]}
      continue;
    }
    if(fence){fence.push(raw);continue}
    if(trimmed && /^(===+|---+)$/.test(next)){
      flush();
      const n=next.startsWith('=')?3:4;
      out.push(`<h${n}>${inline(trimmed)}</h${n}>`);
      i++;
      continue;
    }
    if(/^\s*([-*_])\1{2,}\s*$/.test(trimmed)){flush();out.push('<hr>');continue}
    if(!trimmed){flush();continue}
    const heading=trimmed.match(/^(#{1,6})\s+(.*)$/);
    if(heading){flush();const n=Math.min(heading[1].length+2,6);out.push(`<h${n}>${inline(heading[2])}</h${n}>`);continue}
    const quote=trimmed.match(/^>\s?(.*)$/);
    if(quote){flush();out.push(`<blockquote>${inline(quote[1])}</blockquote>`);continue}
    const ul=trimmed.match(/^[-*+]\s+(.*)$/);
    if(ul){if(!list||list.tag!=='ul'){flush();list={tag:'ul',items:[]}}list.items.push(`<li>${inline(ul[1])}</li>`);continue}
    const ol=trimmed.match(/^\d+\.\s+(.*)$/);
    if(ol){if(!list||list.tag!=='ol'){flush();list={tag:'ol',items:[]}}list.items.push(`<li>${inline(ol[1])}</li>`);continue}
    flush();
    out.push(`<p>${inline(trimmed)}</p>`);
  }
  if(fence) out.push(`<pre><code>${esc(fence.join('\n'))}</code></pre>`);
  flush();
  return out.join('')||esc(cleanReadme(text).slice(0,16000));
}
function updateEntry(p){
  if(!p?.repo) return null;
  return (state.updates||[]).find(x=>x.repo?.owner===p.repo.owner&&x.repo?.name===p.repo.name)||null;
}
function needsUpdate(p){
  if(!p||p.kind!=='github'||!p.installed||!p.repo) return false;
  const u=updateEntry(p);
  if(u) return u.current_sha!==u.available_sha;
  return Boolean(p.available&&p.current&&p.available!==p.current);
}
function filesTouch(files, name){
  return (files||[]).some(f=>f===name||f.endsWith(`/${name}`)||f.includes(`/plugins/${name}`));
}
function stampApplied(preview, applied){
  const sha=applied?.commit||preview?.target_sha;
  if(!preview) return;
  for(const p of state.catalog){
    if(!filesTouch(preview.files, p.name)) continue;
    if(sha){p.current=sha;p.available=sha}
  }
  if(state.selected&&filesTouch(preview.files, state.selected.name)&&sha){
    state.selected.current=sha;
    state.selected.available=sha;
  }
  state.updates=(state.updates||[]).filter(x=>!filesTouch(preview.files, x.name));
}
function workingTheme(){return state.configTheme??state.snapshot?.values.theme??''}
function workingPlugins(){return state.configPlugins??state.snapshot?.values.plugins??[]}
function previewTouches(p){
  if(!state.preview||!p?.name) return false;
  const name=p.name.replace(/[.*+?^${}()|[\]\\]/g,'\\$&');
  const token=new RegExp(`(^|[^A-Za-z0-9_-])${name}([^A-Za-z0-9_-]|$)`);
  if(token.test(state.preview.diff||'')||token.test(state.preview.detail||'')) return true;
  return (state.preview.files||[]).some(f=>f.endsWith(`/${p.name}`)||f.includes(`/plugins/${p.name}`));
}
function setStatus(s,error=false){state.status=s;$('status-text').textContent=s;$('status-dot').classList.toggle('error',error)}
async function call(request){
  if(!native) throw new Error(t('status.readOnly'));
  return tauriInvoke('dispatch',{request});
}
function rememberSelected(){
  if(!state.selected) return;
  const selected=state.selected;
  state.selected=state.catalog.find(p=>p.name===selected.name&&((!p.repo&&!selected.repo)||p.repo?.owner===selected.repo?.owner&&p.repo?.name===selected.repo?.name))||state.selected;
}
function buildCatalog(snapshot){
  const enabled=new Set(snapshot.values.plugins||[]);
  const custom=customCatalog(snapshot);
  const customNames=new Set(custom.map(p=>p.name));
  const official=(snapshot.official_plugins||[]).filter(p=>!customNames.has(p.name)).map(p=>({
    name:p.name, summary:p.summary||'', source:'Oh My Zsh', kind:'official',
    installed:true, enabled:enabled.has(p.name), repo:{owner:'ohmyzsh',name:'ohmyzsh'},
    current:null, available:null, stars:0, updated:null, avatar:null, license:'MIT'
  }));
  const missing=snapshot.values.plugins.filter(name=>!customNames.has(name)&&!official.some(p=>p.name===name)).map(name=>({
    name, summary:'', source:'Oh My Zsh', kind:'configured',
    installed:false, enabled:true, repo:null, current:null, available:null, stars:0, updated:null, avatar:null
  }));
  return [...official, ...custom, ...missing];
}
function customCatalog(snapshot){
  const info=snapshot.installed_custom_info||[];
  return (snapshot.installed_custom||[]).filter(name=>name!=='example'||snapshot.values.plugins.includes(name)).map(name=>{
    const x=info.find(v=>v.name===name)||{};
    const hasRepo=Boolean(x.repo);
    return {
      name, summary:x.summary||'',
      source:hasRepo?`GitHub · ${x.repo.owner}/${x.repo.name}`:'',
      kind:hasRepo?'github':'local', installed:true, enabled:snapshot.values.plugins.includes(name),
      repo:x.repo||{owner:'local',name}, current:x.current_sha||null, stars:0, updated:null, avatar:null, loadable:x.loadable!==false
    };
  });
}
function fixtureSnapshot(){
  return {
    path:'~/.zshrc (fixture)', source:'# Browser preview\nZSH_THEME="robbyrussell"\nplugins=(git brew zsh-autosuggestions)\n',
    values:{theme:'robbyrussell',plugins:['git','brew','zsh-autosuggestions']}, warnings:[], zsh_available:true,
    platform:'browser preview', plugin_root:'~/.oh-my-zsh/custom/plugins',
    installed_custom:['zsh-autosuggestions'],
    installed_custom_info:[{name:'zsh-autosuggestions',repo:{owner:'zsh-users',name:'zsh-autosuggestions'},current_sha:null}],
    official_plugins:[
      {name:'git',summary:'Git aliases and completions'},
      {name:'brew',summary:'Homebrew aliases and completions'},
      {name:'sudo',summary:'Prefix the previous command with sudo'},
      {name:'macos',summary:'macOS command aliases'},
      {name:'docker',summary:'Docker completion and aliases'},
      {name:'extract',summary:'Extract common archive formats'}
    ],
    themes:['robbyrussell']
  };
}
async function read(statusAfter){
  const status=typeof statusAfter==='string'?statusAfter:null;
  if(!native){
    state.snapshot=fixtureSnapshot();
    state.catalog=buildCatalog(state.snapshot);
    rememberSelected();
    render();
    return;
  }
  try{
    const r=await call({kind:'read'});
    state.snapshot=r.snapshot;
    const previous=state.catalog;
    state.catalog=buildCatalog(state.snapshot);
    mergeCatalog(previous);
    rememberSelected();
    state.preview=null;
    if(!status){
      state.configTheme=null;
      state.configPlugins=null;
    }
    setStatus(status||`${t('status.loaded')} ${state.snapshot.path}`);
    render();
  }catch(e){setStatus(e.message,true);render()}
}
function title(){return {overview:[t('title.overview'),t('subtitle.overview')],installed:[t('title.installed'),t('subtitle.installed')],configuration:[t('title.configuration'),t('subtitle.configuration')],discover:[t('title.discover'),t('subtitle.discover')],updates:[t('title.updates'),t('subtitle.updates')],activity:[t('title.activity'),t('subtitle.activity')]}[state.section]}
function render(){
  const list=document.querySelector('.list-panel');
  const y=list?.scrollTop??0;
  applyPreferences();
  const [heading,sub]=title();
  $('page-title').textContent=heading;
  $('page-subtitle').textContent=sub;
  $('global-search').value=state.search;
  $('search-wrap').classList.toggle('is-visible',state.section==='discover'||state.section==='installed');
  const count=$('updates-count');
  if(state.updatesLoading){count.hidden=true}
  else if(state.updates&&state.updates.length){count.hidden=false;count.textContent=String(state.updates.length)}
  else {count.hidden=true}
  ({overview,installed,configuration,discover,updates,activity}[state.section])();
  const next=document.querySelector('.list-panel');
  if(next) next.scrollTop=y;
}
function overview(){
  const s=state.snapshot;
  const warnings=s?.warnings||[];
  const updateLabel=state.updates===null?t('overview.updatesUnchecked'):String(state.updates.length);
  $('content').innerHTML=`<div class="cards">
    <button class="card" data-go="configuration"><svg class="symbol" viewBox="0 0 16 16"><rect x="3" y="2.5" width="10" height="11" rx="1.6" fill="none" stroke="currentColor" stroke-width="1.4"/></svg><div class="value">.zshrc</div><div class="label">${t('overview.configuration')}</div></button>
    <button class="card" data-go="installed"><svg class="symbol" viewBox="0 0 16 16"><circle cx="8" cy="8" r="5.1" fill="none" stroke="currentColor" stroke-width="1.4"/></svg><div class="value">${s?.values.plugins.length??'—'}</div><div class="label">${t('overview.enabled')}</div></button>
    <button class="card" data-go="updates"><svg class="symbol" viewBox="0 0 16 16"><path d="M3.4 8A4.6 4.6 0 0 1 12.2 5.4" fill="none" stroke="currentColor" stroke-width="1.4"/></svg><div class="value">${updateLabel}</div><div class="label">${t('overview.updates')}</div></button>
  </div>
  <section class="group"><div class="group-title">${t('overview.safety')}</div><div class="group-body"><div class="callout"><span class="check">✓</span><span>${t('overview.safetyText')}</span></div></div></section>
  ${warnings.length?`<section class="group warning"><div class="group-title">${t('overview.readOnly')}</div><div class="group-body">${warnings.map(esc).join('<br>')}<br><span class="muted">${t('overview.readOnlyText')}</span></div></section>`:''}`;
  document.querySelectorAll('[data-go]').forEach(b=>b.onclick=()=>go(b.dataset.go));
}
function configuration(){
  const s=state.snapshot;
  if(!s){$('content').innerHTML=`<div class="empty"><strong>${t('config.unavailable')}</strong><span>${t('config.refresh')}</span></div>`;return}
  const selected=new Set(workingPlugins());
  const q=(state.configQuery||'').toLowerCase();
  const names=[...new Set([...workingPlugins(),...state.catalog.filter(p=>isCheckout(p)&&p.loadable!==false).map(p=>p.name)])];
  let candidates=names.map(name=>state.catalog.find(p=>p.name===name)||{name,summary:t('installed.configuredPlugin'),kind:'configured',installed:false});
  candidates.sort((a,b)=>(selected.has(b.name)-selected.has(a.name))||a.name.localeCompare(b.name));
  if(q) candidates=candidates.filter(p=>`${p.name} ${pluginSummary(p)}`.toLowerCase().includes(q));
  const currentTheme=workingTheme();
  const themes=[...new Set([currentTheme,...(s.themes||[])].filter(Boolean))];
  $('content').innerHTML=`<section class="group"><div class="group-title">${t('config.recognized')}</div><div class="group-body">
    <p class="muted">${esc(s.path)} · ${s.zsh_available?t('config.zshReady'):t('config.zshMissing')}</p>
    <div class="form-row"><label>${t('config.theme')}</label><select id="theme-input" class="select-input">${themes.map(theme=>`<option value="${esc(theme)}" ${theme===currentTheme?'selected':''}>${esc(theme)}</option>`).join('')}</select></div>
    <p class="hint">${t('config.themeHint')}</p>
    <label class="field-label" for="plugin-filter">${t('config.plugins')}</label>
    <input id="plugin-filter" class="text-input config-search" value="${esc(state.configQuery)}" placeholder="${esc(t('config.pluginSearch'))}" />
    <p class="hint">${t('config.pluginsHint')}</p>
    <div class="plugin-switches">${candidates.map(p=>`<label class="switch-row"><input type="checkbox" data-config-plugin value="${esc(p.name)}" ${selected.has(p.name)?'checked':''} /><span class="switch-copy"><strong>${esc(p.name)}</strong><small>${esc(pluginSummary(p))}</small></span>${p.kind==='official'?`<span class="badge">${t('common.official')}</span>`:p.installed||isCheckout(p)?`<span class="badge">${t('common.installed')}</span>`:''}</label>`).join('')||`<div class="empty"><strong>${t('common.noResults')}</strong></div>`}</div>
    <div class="actions"><button class="secondary" id="preview-config">${t('config.preview')}</button>${state.preview?`<button class="primary" id="apply-config">${t('config.apply')}</button>`:''}</div>
    ${state.preview?`<pre class="diff">${diffHTML(state.preview.diff)}</pre>`:''}
  </div></section>
  <section class="group"><div class="group-title">${t('config.raw')}</div><div class="group-body"><details><summary>${t('config.showRaw')}</summary><pre class="source">${esc(s.source)}</pre></details></div></section>`;
  $('plugin-filter').oninput=e=>{state.configQuery=e.target.value;render()};
  $('theme-input').onchange=e=>{state.configTheme=e.target.value;previewConfig()};
  document.querySelectorAll('[data-config-plugin]').forEach(box=>{
    box.onchange=()=>{
      const name=box.value;
      const next=workingPlugins().filter(n=>n!==name);
      if(box.checked){
        const item=state.catalog.find(p=>p.name===name);
        if(item&&item.loadable===false){
          box.checked=false;
          setStatus(t('status.notOmzPlugin'),true);
          return;
        }
        next.push(name);
      }
      state.configPlugins=next;
      previewConfig();
    };
  });
  $('preview-config').onclick=previewConfig;
  if($('apply-config')) $('apply-config').onclick=()=>confirmApply(t('config.apply'),state.preview?.detail||t('status.preview'),applyPreview);
}
function installedRows(){return state.catalog.filter(isInUse)}
function pluginRow(p, selected){
  return `<button class="plugin-row ${selected?'selected':''}" data-plugin="${esc(p.name)}" data-owner="${esc(p.repo?.owner||'')}" type="button">
    <span class="plugin-leading">${pluginIcon(p)}</span>
    <span class="plugin-copy"><h3><span class="name">${esc(p.name)}</span>${p.enabled?`<span class="badge orange">${t('common.enabled')}</span>`:''}</h3>
    <p>${esc(pluginSummary(p))}</p>
    <div class="meta"><span>${esc(p.kind==='official'?t('common.official'):pluginSource(p))}</span>${categoryBadge(p)}${starMetric(p.stars)}${p.updated?`<span>${esc(formatDate(p.updated))}</span>`:''}${isCheckout(p)?`<span class="badge">${t('common.installed')}</span>`:''}${externalRepo(p,true)}</div>
    </span></button>`;
}
function bindPluginList(rows){
  document.querySelectorAll('[data-plugin]').forEach(button=>{
    button.onclick=e=>{
      if(e.target.closest('[data-open-url]')) return;
      const p=rows.find(x=>x.name===button.dataset.plugin&&(x.repo?.owner||'')===button.dataset.owner)||state.catalog.find(x=>x.name===button.dataset.plugin);
      if(p) selectPlugin(p);
    };
  });
  bindDetailActions();
}
function installed(){
  const q=state.search.toLowerCase();
  const rows=installedRows().filter(p=>!q||`${p.name} ${pluginSummary(p)} ${pluginSource(p)} ${categoryLabel(p)}`.toLowerCase().includes(q));
  $('content').innerHTML=`<div class="split"><section class="list-panel">${rows.map(p=>pluginRow(p,state.selected?.name===p.name&&(state.selected?.repo?.owner||'')===(p.repo?.owner||''))).join('')||`<div class="empty"><strong>${t('installed.empty')}</strong><span>${t('installed.start')}</span></div>`}</section><section class="detail-panel">${state.selected?detail(state.selected):`<div class="empty"><strong>${t('installed.select')}</strong><span>${t('installed.details')}</span></div>`}</section></div>`;
  bindPluginList(rows);
}
async function previewConfig(){
  const theme=($('theme-input')?.value.trim())||workingTheme();
  const plugins=state.configPlugins||[...document.querySelectorAll('[data-config-plugin]:checked')].map(x=>x.value);
  state.configTheme=theme;
  state.configPlugins=plugins;
  try{
    if(!native){setStatus(t('status.readOnly'));return}
    const r=await call({kind:'preview_config',theme,plugins,expected_source:state.snapshot.source});
    state.preview=r.preview;
    setStatus(t('status.preview'));
    render();
  }catch(e){setStatus(e.message,true)}
}
async function applyPreview(){await applyAny()}
async function applyAny(){
  if(!state.preview) return;
  if(!native){setStatus(t('status.readOnly'),true);return}
  try{
    const preview=state.preview;
    const r=await call({kind:'apply',plan_id:preview.id});
    const message=`${t('status.applied')} · ${r.applied.status}${r.applied.backup?` · ${r.applied.backup}`:''}`;
    stampApplied(preview, r.applied);
    state.preview=null;
    state.configTheme=null;
    state.configPlugins=null;
    await read(message);
    try{
      await call({kind:'open_terminal'});
      setStatus(`${message} · ${t('status.newTerminal')}`);
    }catch{
      setStatus(`${message} · ${t('status.reloadHint')}`);
    }
    state.updatesLoading=true;
    render();
    if(native) await checkUpdates();
  }catch(e){setStatus(e.message,true)}
}
function discover(){
  if(native&&!state.recommendedLoaded&&!state.recommendationsLoading){state.recommendationsLoading=true;loadRecommendations()}
  const q=state.search.toLowerCase();
  let rows=state.catalog.filter(p=>{
    const matchesSearch=!q||`${p.name} ${pluginSummary(p)} ${pluginSource(p)} ${categoryLabel(p)}`.toLowerCase().includes(q);
    const matchesSource=state.sourceFilter==='all'||(state.sourceFilter==='github'&&(p.kind==='github'||p.kind==='local'))||(state.sourceFilter==='official'&&(p.kind==='official'||p.kind==='configured'));
    const matchesInstall=state.installFilter==='all'||(state.installFilter==='installed'&&isInUse(p))||(state.installFilter==='available'&&!isInUse(p));
    const matchesCategory=state.categoryFilter==='all'||pluginCategory(p)===state.categoryFilter;
    return matchesSearch&&matchesSource&&matchesInstall&&matchesCategory;
  });
  rows=[...rows].sort((a,b)=>{
    if(state.sort==='updated') return (Date.parse(b.updated||'')||0)-(Date.parse(a.updated||'')||0)||a.name.localeCompare(b.name);
    if(state.sort==='name') return a.name.localeCompare(b.name);
    return (b.stars||0)-(a.stars||0)||a.name.localeCompare(b.name);
  });
  $('content').innerHTML=`<div class="discover-controls">
    <label>${t('discover.source')} <select id="source-filter" class="select-input"><option value="all">${t('discover.allSources')}</option><option value="official">${t('discover.official')}</option><option value="github">${t('discover.github')}</option></select></label>
    <label>${t('discover.availability')} <select id="install-filter" class="select-input"><option value="all">${t('discover.allPlugins')}</option><option value="available">${t('discover.available')}</option><option value="installed">${t('discover.installed')}</option></select></label>
    <label>${t('discover.category')} <select id="category-filter" class="select-input"><option value="all">${t('discover.allCategories')}</option>${PLUGIN_CATEGORIES.map(id=>`<option value="${id}">${t('category.'+id)}</option>`).join('')}</select></label>
    <label>${t('discover.sort')} <select id="sort-filter" class="select-input"><option value="stars">${t('discover.stars')}</option><option value="updated">${t('discover.updated')}</option><option value="name">${t('discover.name')}</option></select></label>
    ${state.recommendationsLoading?`<span class="muted">${t('discover.loading')}</span>`:''}
  </div>
  <p class="discover-summary">${rows.length} ${rows.length===1?t('discover.result'):t('discover.results')}${q?` · ${esc(state.search)}`:''} · ${t('discover.metadata')}</p>
  <div class="split"><section class="list-panel">${rows.map(p=>pluginRow(p,state.selected?.name===p.name&&(state.selected?.repo?.owner||'')===(p.repo?.owner||''))).join('')||`<div class="empty"><strong>${t('common.noResults')}</strong><span>${t('common.adjustFilters')}</span></div>`}</section><section class="detail-panel">${state.selected?detail(state.selected):`<div class="empty"><strong>${t('common.selectPlugin')}</strong><span>${t('common.detailsActions')}</span></div>`}</section></div>`;
  $('source-filter').value=state.sourceFilter;$('install-filter').value=state.installFilter;$('category-filter').value=state.categoryFilter;$('sort-filter').value=state.sort;
  $('source-filter').onchange=e=>{state.sourceFilter=e.target.value;render()};
  $('install-filter').onchange=e=>{state.installFilter=e.target.value;render()};
  $('category-filter').onchange=e=>{state.categoryFilter=e.target.value;render()};
  $('sort-filter').onchange=e=>{state.sort=e.target.value;render()};
  bindPluginList(rows);
}
function detail(p){
  const repo=p.kind==='official'?`ohmyzsh/ohmyzsh/plugins/${p.name}`:(p.repo?`${p.repo.owner}/${p.repo.name}`:'');
  const remote=p.kind==='github'&&p.repo&&p.repo.owner!=='local';
  const lifecycle=remote
    ?(needsUpdate(p)?`<button class="primary" id="plugin-action">${t('common.reviewUpdate')}</button>`
      :p.installed?'':`<button class="primary" id="plugin-action">${t('common.reviewInstall')}</button>`)
    :'';
  const enable=p.loadable===false?'':`<button class="secondary" id="plugin-enable">${p.enabled?t('common.disable'):t('common.enable')}</button>`;
  const remove=p.kind==='github'||p.kind==='local'?`<button class="secondary" id="plugin-remove">${t('common.reviewUninstall')}</button>`:'';
  const readme=p.readme
    ?`<section class="readme-section"><div class="readme-title">${t('detail.readme')}</div><article class="readme">${renderReadme(p.readme)}</article></section>`
    :p.readmeLoading?`<p class="readme-loading">${t('detail.loadingReadme')}</p>`
    :p.readmeError?`<p class="readme-error">${t('detail.readmeError')}</p>`
    :`<button class="secondary" id="load-readme">${t('detail.loadReadme')}</button>`;
  return `<div class="detail-heading"><span class="detail-icon">${pluginIcon(p)}</span><div><h2>${esc(p.name)}</h2><p class="summary">${esc(pluginSummary(p))}</p></div></div>
    <dl class="detail-grid">
      <dt>${t('detail.source')}</dt><dd>${esc(pluginSource(p))}</dd>
      <dt>${t('detail.category')}</dt><dd>${categoryBadge(p)}</dd>
      <dt>${t('detail.repository')}</dt><dd>${esc(repo||t('detail.bundled'))} ${externalRepo(p)}</dd>
      <dt>${t('detail.stars')}</dt><dd>${p.stars?starMetric(p.stars):'—'}</dd>
      <dt>${t('detail.license')}</dt><dd>${esc(p.license||'—')}</dd>
      <dt>${t('detail.updated')}</dt><dd>${esc(formatDate(p.updated))}</dd>
      <dt>${t('detail.currentSha')}</dt><dd>${esc(p.current?p.current.slice(0,8)+'…':t('detail.notRecorded'))}</dd>
    </dl>
    <hr><div class="actions">${lifecycle}${enable}${remove}</div>
    ${previewTouches(p)?`<pre class="diff">${diffHTML(state.preview.diff)}\n\n${esc(state.preview.detail)}</pre><div class="actions"><button class="primary" id="apply-plugin">${t('common.apply')}</button><button class="secondary" id="cancel-preview">${t('common.cancel')}</button></div>`:''}
    ${readme}<p class="hint">${t('detail.safety')}</p>`;
}
function bindDetailActions(){
  if($('load-readme')) $('load-readme').onclick=()=>loadReadme(state.selected);
  if($('plugin-enable')) $('plugin-enable').onclick=togglePlugin;
  if($('plugin-action')) $('plugin-action').onclick=()=>previewPlugin(state.selected.kind==='github'&&state.selected.installed?'update':'install');
  if($('plugin-remove')) $('plugin-remove').onclick=()=>previewPlugin('remove');
  if($('apply-plugin')) $('apply-plugin').onclick=()=>confirmApply(t('common.apply'),state.preview.detail,applyAny);
  if($('cancel-preview')) $('cancel-preview').onclick=()=>{state.preview=null;state.configTheme=null;state.configPlugins=null;render()};
}
function selectPlugin(p){
  state.selected=p;
  render();
  if(native&&p&&!p.readme&&!p.readmeLoading&&!p.readmeError) loadReadme(p);
}
async function loadReadme(p){
  if(!p||p.readmeLoading) return;
  if(!native){p.readmeError=true;render();return}
  p.readmeLoading=true;render();
  try{
    const r=p.kind==='github'&&p.repo&&p.repo.owner!=='local'
      ?await call({kind:'readme',owner:p.repo.owner,repository:p.repo.name})
      :await call({kind:'plugin_readme',name:p.name});
    p.readme=r.message||'';
    p.readmeError=false;
  }catch{p.readmeError=true}
  finally{p.readmeLoading=false;render()}
}
async function searchGitHub(query=null){
  const q=(query??state.search).trim()||'zsh plugin';
  try{
    setStatus(`${t('status.searching')} ${q}…`);
    const r=await call({kind:'search',query:q});
    for(const x of r.search.items){
      const existing=state.catalog.find(p=>p.repo?.name===x.repo.name&&p.repo?.owner===x.repo.owner);
      if(existing) Object.assign(existing,{summary:x.summary,stars:x.stars,license:x.license,updated:x.updated,avatar:x.avatar,available:null,source:`GitHub · ${x.repo.owner}/${x.repo.name}`});
      else state.catalog.push({name:x.repo.name,summary:x.summary,source:`GitHub · ${x.repo.owner}/${x.repo.name}`,kind:'github',installed:false,enabled:false,repo:x.repo,license:x.license,stars:x.stars,updated:x.updated,avatar:x.avatar,current:null,available:null});
    }
    if(q==='zsh plugin') state.recommendedLoaded=true;
    setStatus(`${t('status.githubChecked')} ${r.search.remaining??'unknown'}`);
    render();
  }catch(e){state.recommendationsLoading=false;setStatus(e.message,true);render()}
}
async function loadRecommendations(){try{await searchGitHub('zsh plugin')}finally{state.recommendationsLoading=false;render()}}
async function previewPlugin(operation,p=state.selected){
  if(!p?.repo||p.kind==='official'){setStatus(t('status.officialEnable'),true);return}
  try{
    const r=await call({kind:'preview_plugin',operation,owner:p.repo.owner,repository:p.repo.name});
    state.preview=r.preview;
    setStatus(t('status.review'));
    render();
  }catch(e){setStatus(e.message,true)}
}
async function togglePlugin(){
  const p=state.selected;
  if(!p||!state.snapshot) return;
  if(!native){setStatus(t('status.readOnly'));return}
  const plugins=state.snapshot.values.plugins.filter(name=>name!==p.name);
  if(!p.enabled){
    if(p.kind==='github'&&!p.installed){setStatus(t('status.installFirst'),true);return}
    if(p.loadable===false){setStatus(t('status.notOmzPlugin'),true);return}
    plugins.push(p.name);
  }
  try{
    const r=await call({kind:'preview_config',theme:state.snapshot.values.theme,plugins,expected_source:state.snapshot.source});
    state.preview=r.preview;
    setStatus(t('status.preview'));
    render();
    confirmApply(p.enabled?t('common.disable'):t('common.enable'),`${p.name} · ${t('status.preview')}`,applyAny);
  }catch(e){setStatus(e.message,true)}
}
function diffHTML(s){return esc(s).split('\n').map(x=>x.startsWith('+')?`<span class="add">${x}</span>`:x.startsWith('-')?`<span class="remove">${x}</span>`:x).join('\n')}
function updates(){
  if(state.updates===null&&!state.updatesLoading){
    state.updatesLoading=true;
    if(native) checkUpdates();
    else {state.updates=[];state.updatesLoading=false}
  }
  const rows=state.updates||[];
  $('content').innerHTML=`<div class="actions" style="margin-top:0"><button class="secondary" id="refresh-updates">${t('updates.check')}</button></div>
    ${state.updatesLoading?`<div class="empty"><strong>${t('updates.checking')}</strong><span>${t('updates.compare')}</span></div>`
    :rows.length?`<section class="group"><div class="group-title">${t('updates.available')}</div><div class="group-body update-list">${rows.map((x,i)=>`<button class="update-row" data-update="${i}"><strong>${esc(x.name)}</strong><span>${esc(x.repo.owner+'/'+x.repo.name)}</span><small>${esc(x.current_sha.slice(0,8)+'…')} → ${esc(x.available_sha.slice(0,8)+'…')}</small><p>${esc(x.summary||'')}</p></button>`).join('')}</div></section>`
    :`<div class="empty"><strong>${t('updates.none')}</strong><span>${t('updates.noneText')}</span></div>`}`;
  if($('refresh-updates')) $('refresh-updates').onclick=()=>{state.updates=null;render()};
  document.querySelectorAll('[data-update]').forEach(b=>b.onclick=()=>{
    const x=rows[+b.dataset.update];
    let p=state.catalog.find(y=>y.repo?.owner===x.repo.owner&&y.repo?.name===x.repo.name);
    if(!p){p={name:x.name,summary:'',source:`GitHub · ${x.repo.owner}/${x.repo.name}`,kind:'github',installed:true,enabled:state.snapshot.values.plugins.includes(x.name),repo:x.repo,current:x.current_sha,available:x.available_sha,avatar:null};state.catalog.push(p)}
    else {p.current=x.current_sha;p.available=x.available_sha}
    state.selected=p;go('discover');
  });
}
async function checkUpdates(){
  try{
    const r=await call({kind:'updates'});
    state.updates=r.updates||[];
    for(const p of state.catalog){
      if(p.kind!=='github'||!p.repo) continue;
      const u=state.updates.find(x=>x.repo?.owner===p.repo.owner&&x.repo?.name===p.repo.name);
      if(u){p.current=u.current_sha;p.available=u.available_sha}
      else if(p.installed) p.available=p.current;
    }
  }catch(e){setStatus(e.message,true);state.updates=[]}
  finally{state.updatesLoading=false;render()}
}
function activity(){
  $('content').innerHTML=`<div class="empty"><strong>${t('activity.heading')}</strong><span>${t('activity.lede')}</span></div>`;
  loadActivity();
}
async function loadActivity(){
  let rows=[];
  if(native) try{rows=(await call({kind:'history'})).history||[]}catch{}
  if(state.section!=='activity') return;
  try{
  const backups=[...new Map(rows.filter(x=>x.backup).map(x=>[x.backup,x])).values()];
  $('content').innerHTML=`${backups.length?`<section class="group"><div class="group-title">${t('activity.backups')}</div><div class="group-body backup-list">${backups.map(x=>`<div class="backup-row"><div><strong>${esc(formatTimestamp(x.at))}</strong><small>${esc(pathName(x.backup))} · ${esc(x.backup)}</small></div><div class="actions"><button class="secondary" data-restore-backup="${esc(x.backup)}">${t('activity.restore')}</button><button class="secondary" data-delete-backup="${esc(x.backup)}">${t('activity.delete')}</button></div></div>`).join('')}</div></section>`:''}
    ${rows.length?rows.map(x=>`<section class="group"><div class="group-title">${esc(historyLabel(x.operation))} · ${esc(historyLabel(x.status))}</div><div class="group-body"><span class="muted">${esc(formatTimestamp(x.at))}</span>${x.repo?`<br>${esc(x.repo.owner+'/'+x.repo.name)}`:''}${x.commit?`<br>${t('activity.commit')} <code>${esc(x.commit)}</code>`:''}${x.previous_commit?`<br>${t('activity.previousCommit')} <code>${esc(x.previous_commit)}</code>`:''}<br>${esc(x.files.join('\n'))}${x.backup?`<br><span class="muted">${t('activity.backupLabel')}: ${esc(x.backup)}</span>`:''}${x.undo_available?`<div class="actions"><button class="secondary" data-undo="${esc(x.id)}">${t('activity.undo')}</button></div>`:''}</div></section>`).join(''):`<div class="empty"><strong>${t('activity.none')}</strong><span>${t('activity.noneText')}</span></div>`}`;
  document.querySelectorAll('[data-undo]').forEach(b=>b.onclick=()=>confirmApply(t('activity.restoreTitle'),t('activity.restoreText'),()=>undoHistory(b.dataset.undo)));
  document.querySelectorAll('[data-restore-backup]').forEach(b=>b.onclick=()=>confirmApply(t('activity.restoreTitle'),t('activity.restoreText'),()=>restoreBackup(b.dataset.restoreBackup)));
  document.querySelectorAll('[data-delete-backup]').forEach(b=>b.onclick=()=>confirmApply(t('activity.delete'),b.dataset.deleteBackup,()=>deleteBackup(b.dataset.deleteBackup)));
  }catch(e){
    $('content').innerHTML=`<div class="empty"><strong>${t('activity.heading')}</strong><span>${esc(e.message)}</span></div>`;
  }
}
async function restoreBackup(path){if(!native){setStatus(t('status.readOnly'),true);return}try{const r=await call({kind:'restore_backup',path});await read(r.applied.status)}catch(e){setStatus(e.message,true)}}
async function deleteBackup(path){if(!native){setStatus(t('status.readOnly'),true);return}try{await call({kind:'delete_backup',path});setStatus(t('activity.delete'));render()}catch(e){setStatus(e.message,true)}}
async function undoHistory(id){if(!native){setStatus(t('status.readOnly'),true);return}try{const r=await call({kind:'undo',history_id:id});await read(r.applied.status+(r.applied.backup?` · ${r.applied.backup}`:''))}catch(e){setStatus(e.message,true)}}
function confirmApply(title,message,fn){
  const d=$('confirm-dialog');
  $('confirm-title').textContent=title;
  $('confirm-message').textContent=message;
  d.showModal();
  d.addEventListener('close',async function h(){d.removeEventListener('close',h);if(d.returnValue==='confirm') await fn()},{once:true});
}
function go(section){
  state.section=section;
  if(section==='installed'&&state.selected&&!isInUse(state.selected)) state.selected=null;
  document.querySelectorAll('.nav-item').forEach(x=>x.classList.toggle('active',x.dataset.section===section));
  render();
}
async function openExternal(url){
  if(!url) return;
  setStatus(t('status.opening'));
  if(native){
    try{await call({kind:'open_url',url})}
    catch(e){setStatus(e.message,true);return}
  } else window.open(url,'_blank','noreferrer');
}
$('content').addEventListener('click',e=>{
  const link=e.target.closest('[data-open-url]');
  if(!link) return;
  e.preventDefault();
  e.stopPropagation();
  openExternal(link.dataset.openUrl);
});
$('global-search').oninput=e=>{state.search=e.target.value;render()};
$('global-search').onkeydown=e=>{if(e.key==='Enter'&&state.section==='discover') searchGitHub()};
function toggleSidebar(){state.sidebarCollapsed=!state.sidebarCollapsed;savePreference('sidebar',state.sidebarCollapsed?'collapsed':'open');applyPreferences()}
$('sidebar-toggle').onclick=toggleSidebar;
$('sidebar-inline-toggle').onclick=toggleSidebar;
$('refresh-button').onclick=read;
function refreshTokenStatus(){if(!native)return;call({kind:'token_status'}).then(r=>{$('token-status').textContent=r.message==='configured'?t('settings.tokenConfigured'):r.message==='not_configured'?t('settings.tokenNone'):t('settings.tokenUnavailable')}).catch(x=>{$('token-status').textContent=x.message})}
$('settings-button').onclick=()=>{
  translateStatic();
  $('language-select').value=state.language;
  $('appearance-select').value=state.appearance;
  $('glass-toggle').checked=state.glass;
  $('settings-dialog').showModal();
  $('token-status').textContent=t('settings.tokenPrivate');
  window.setTimeout(refreshTokenStatus,0);
};
document.addEventListener('keydown',e=>{
  if((e.metaKey||e.ctrlKey) && (e.key===','||e.code==='Comma')){
    e.preventDefault();
    $('settings-button').click();
  }
});
$('language-select').onchange=e=>{state.language=e.target.value;savePreference('language',state.language);if(state.snapshot) setStatus(`${t('status.loaded')} ${state.snapshot.path}`);render()};
$('appearance-select').onchange=e=>{state.appearance=e.target.value;savePreference('appearance',state.appearance);applyPreferences();render()};
$('glass-toggle').onchange=e=>{state.glass=e.target.checked;savePreference('glass',state.glass?'on':'off');applyPreferences()};
$('settings-form').onsubmit=async e=>{
  if(e.submitter?.value!=='save') return;
  state.language=$('language-select').value;
  state.appearance=$('appearance-select').value;
  state.glass=$('glass-toggle').checked;
  savePreference('language',state.language);
  savePreference('appearance',state.appearance);
  savePreference('glass',state.glass?'on':'off');
  applyPreferences();
  if(native){
    try{
      await call({kind:'set_token',token:$('github-token').value.trim()});
      $('token-status').textContent=$('github-token').value.trim()?t('settings.tokenConfigured'):t('settings.tokenNone');
      setStatus(t('settings.preferencesSaved'));
    }catch(x){setStatus(x.message,true)}
  } else setStatus(t('settings.preferencesSaved'));
};
document.querySelectorAll('.nav-item').forEach(b=>b.onclick=()=>go(b.dataset.section));
applyPreferences();
read();
