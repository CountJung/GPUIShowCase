const hero = document.querySelector("#hero");
const routeList = document.querySelector("#route-list");
const harnessList = document.querySelector("#harness-list");
const targetList = document.querySelector("#target-list");
const debugList = document.querySelector("#debug-list");

const renderChips = (items) =>
  `<div class="meta-chip-row">${items
    .map((item) => `<span class="meta-chip">${item}</span>`)
    .join("")}</div>`;

const renderEntry = (entry, delayIndex = 0) => `
  <article class="entry-card" style="animation-delay:${delayIndex * 70}ms">
    <div class="entry-title-row">
      <div class="entry-title">${entry.title}</div>
      ${entry.status ? `<span class="status-badge ${entry.statusClass || ""}">${entry.status}</span>` : ""}
    </div>
    ${entry.summary ? `<div class="entry-subtext">${entry.summary}</div>` : ""}
    ${entry.meta?.length ? renderChips(entry.meta) : ""}
  </article>
`;

const renderHero = (data) => {
  hero.innerHTML = `
    <div class="hero-top">
      <div>
        <span class="eyebrow">Independent Web Mode</span>
        <h1>${data.name}</h1>
      </div>
      <div class="meta-chip-row">
        <span class="meta-chip">native desktop first</span>
        <span class="meta-chip">browser companion</span>
        <span class="meta-chip">VS Code ready</span>
      </div>
    </div>
    <p class="hero-summary">${data.summary}</p>
    <div class="hero-metrics">
      ${data.metrics
        .map(
          (metric) => `
            <div class="metric-card">
              <div class="eyebrow">${metric.label}</div>
              <div class="metric-value">${metric.value}</div>
              <div class="entry-subtext">${metric.note}</div>
            </div>
          `,
        )
        .join("")}
    </div>
  `;
};

const renderSection = (container, entries) => {
  container.innerHTML = entries
    .map((entry, index) => renderEntry(entry, index))
    .join("");
};

fetch("/data/showcase.json")
  .then((response) => {
    if (!response.ok) {
      throw new Error(`failed to load manifest: ${response.status}`);
    }
    return response.json();
  })
  .then((data) => {
    renderHero(data);
    renderSection(routeList, data.routes);
    renderSection(harnessList, data.harnesses);
    renderSection(targetList, data.targets);
    renderSection(debugList, data.debugProfiles);
  })
  .catch((error) => {
    hero.innerHTML = `
      <span class="eyebrow">Independent Web Mode</span>
      <h1>Web manifest load failed</h1>
      <p class="hero-summary">${error.message}</p>
    `;
  });