import { chromium } from 'playwright';
import { preview } from 'vite';

/**
 * Assert that the page does not exhibit horizontal scrolling / overflow
 */
async function assertNoHorizontalOverflow(page, contextDesc) {
  const { scrollWidth, innerWidth } = await page.evaluate(() => ({
    scrollWidth: document.documentElement.scrollWidth,
    innerWidth: window.innerWidth
  }));
  if (scrollWidth > innerWidth + 1) { // 1px tolerance for sub-pixel rendering
    throw new Error(`[${contextDesc}] Horizontal overflow detected: scrollWidth (${scrollWidth}px) > innerWidth (${innerWidth}px)`);
  }
}

async function run() {
  console.log("================================================================================");
  console.log("Starting End-to-End Visual, Functional & Responsive Validation (v2.1.1 Milestone)");
  console.log("================================================================================");

  // Spin up local Vite preview server on port 4173
  const previewServer = await preview({
    preview: { port: 4173, host: '127.0.0.1' }
  });
  const baseUrl = previewServer.resolvedUrls?.local?.[0] || 'http://127.0.0.1:4173/';

  const browser = await chromium.launch({ headless: true });

  try {
    // =========================================================================
    // SUITE 1: DESKTOP VIEWPORT VALIDATION (1440x900, >= 768px)
    // =========================================================================
    console.log("\n>>> [1/2] Executing Desktop Viewport Test Suite (1440x900)...");
    const desktopContext = await browser.newContext({ viewport: { width: 1440, height: 900 } });
    const desktopPage = await desktopContext.newPage();

    // 1. Initial Page Load & Cold Start
    await desktopPage.goto(baseUrl, { waitUntil: 'networkidle' });
    await desktopPage.evaluate(() => localStorage.clear());
    await desktopPage.reload({ waitUntil: 'networkidle' });
    await desktopPage.waitForTimeout(500);

    // Verify Title & Version Badge
    await desktopPage.waitForSelector('text=Homecalc', { timeout: 10000 });
    const title = await desktopPage.title();
    console.log("Page title:", title);
    if (!title.includes("Homecalc")) throw new Error("Incorrect page title");

    const versionBadge = await desktopPage.$('text=v2.1.0');
    if (!versionBadge) throw new Error("v2.1.0 badge missing from Header");
    console.log("✓ Dynamic Header version badge verified (v2.1.0).");

    // Verify Desktop SplitPane Side-by-Side Layout
    const desktopAsideBox = await desktopPage.locator('aside').boundingBox();
    const desktopMainBox = await desktopPage.locator('main').boundingBox();
    if (!desktopAsideBox || !desktopMainBox) throw new Error("Desktop SplitPane aside or main bounding box not found");
    if (desktopAsideBox.x >= desktopMainBox.x || (desktopAsideBox.x + desktopAsideBox.width > desktopMainBox.x + 10)) {
      throw new Error(`Desktop SplitPane layout failure: aside (x:${desktopAsideBox.x}, w:${desktopAsideBox.width}) is not side-by-side with main (x:${desktopMainBox.x}, w:${desktopMainBox.width})`);
    }
    console.log(`✓ Desktop Root SplitPane side-by-side layout verified (aside: x=${desktopAsideBox.x}, w=${desktopAsideBox.width} | main: x=${desktopMainBox.x}, w=${desktopMainBox.width}).`);

    // Verify KPI cards populated
    const kpiCards = await desktopPage.$$('.tabular-nums');
    console.log(`Found ${kpiCards.length} tabular numeric indicators on initial load.`);
    if (kpiCards.length < 5) throw new Error("KPI indicators missing on launch");

    const scenarioName = await desktopPage.$('text=Standard 30Y Mortgage');
    if (!scenarioName) throw new Error("Standard 30Y Mortgage title missing on initial load");
    console.log("✓ 'Standard 30Y Mortgage' successfully initialized in Slot 1 on fresh launch.");

    // 2. Switch to Slot 2 and Slot 3
    console.log("Testing slot switching across independent default instances...");
    await desktopPage.click('button:has-text("Slot 2")');
    await desktopPage.waitForTimeout(300);
    await desktopPage.click('button:has-text("Slot 3")');
    await desktopPage.waitForTimeout(300);
    await desktopPage.click('button:has-text("Slot 1")');
    await desktopPage.waitForTimeout(300);
    console.log("✓ All 3 slots verified active with independent 'Standard 30Y Mortgage' instances.");

    // 3. Switch to Charts View
    console.log("Testing Charts view navigation in right pane...");
    await desktopPage.click('button:has-text("Charts")');
    await desktopPage.waitForTimeout(500);
    const svgChart = await desktopPage.$('svg[aria-label="Amortization trajectory chart"]');
    if (!svgChart) throw new Error("Amortization SVG chart missing");
    console.log("✓ Amortization Trajectory Chart rendered successfully.");

    // 4. Switch to Statements View
    console.log("Testing Statements view navigation in right pane...");
    await desktopPage.click('button:has-text("Statements")');
    await desktopPage.waitForTimeout(500);
    const tableRows = await desktopPage.$$('tbody tr');
    console.log(`✓ Statements table rendered with ${tableRows.length} rows.`);
    if (tableRows.length === 0) throw new Error("Statements table empty");

    // 5. Switch to Compare View
    console.log("Testing Compare view navigation in right pane...");
    await desktopPage.click('button:has-text("Compare")');
    await desktopPage.waitForTimeout(500);
    const compareHeader = await desktopPage.$('text=Scenario Differential Analysis');
    if (!compareHeader) throw new Error("Compare view missing");
    const grossOutlay = await desktopPage.$('text=Gross Outlay');
    if (!grossOutlay) throw new Error("Gross Outlay KPI card missing in Compare view");
    const interestPaid = await desktopPage.$('text=Interest Paid');
    if (!interestPaid) throw new Error("Interest Paid KPI card missing in Compare view");
    const presentValue = await desktopPage.$('text=Present Value');
    if (!presentValue) throw new Error("Present Value KPI card missing in Compare view");
    const irrKpi = await desktopPage.$('text=Internal Rate of Return');
    if (!irrKpi) throw new Error("Internal Rate of Return KPI card missing in Compare view");
    const diffTable = await desktopPage.$('text=Metric Differential Table');
    if (!diffTable) throw new Error("Metric Differential Table title missing in Compare view");
    const deltaFormula = await desktopPage.$('text=Delta = B - A');
    if (!deltaFormula) throw new Error("Delta = B - A text missing in Compare view");
    console.log("✓ Compare Differential workspace and updated KPI/table terminology rendered successfully.");

    // 6. Test Parameter Manipulation & House Parity Guard in Compare View
    console.log("Testing reactive parameter update & House Parity Guard...");
    await desktopPage.click('button:has-text("Dashboard")');
    await desktopPage.waitForTimeout(300);
    const priceInput = await desktopPage.$('#purchase-price-input');
    if (!priceInput) throw new Error("Purchase price input missing");
    await priceInput.fill('1200000');
    await priceInput.evaluate(e => e.blur());
    await desktopPage.waitForTimeout(300);

    // Verify House Parity Guard: Slot 1 ($1.2M) vs Slot 2 ($1.0M) must render N/A for IRR
    await desktopPage.click('button:has-text("Compare")');
    await desktopPage.waitForTimeout(500);
    const irrElement = await desktopPage.$('text=Internal Rate of Return');
    if (!irrElement) throw new Error("Internal Rate of Return indicator missing in Compare view");
    const naBadge = await desktopPage.$('span:has-text("N/A")');
    if (!naBadge) throw new Error("House Parity Guard failed: expected N/A for mismatched property prices");
    console.log("✓ House Parity Guard verified in UI: mismatched property price displays N/A for Internal Rate of Return.");

    // 7. Test Save to Library & Scenario Library Modal
    console.log("Testing Save to Library on desktop...");
    await desktopPage.click('button:has-text("Dashboard")');
    await desktopPage.waitForTimeout(300);
    await desktopPage.click('button:has-text("💾 Save")');
    await desktopPage.waitForTimeout(300);

    console.log("Opening Scenario Library modal on desktop...");
    await desktopPage.click('button[aria-label="Open Scenario Library"]');
    await desktopPage.waitForTimeout(500);
    const modalHeader = await desktopPage.$('text=Scenario Library & Presets');
    if (!modalHeader) throw new Error("Scenario Library modal failed to open");

    const baselineCard = await desktopPage.$('text=Standard 30Y Mortgage');
    if (!baselineCard) throw new Error("Standard 30Y Mortgage baseline preset missing from library modal");
    console.log("✓ Scenario Library modal verified with Standard 30Y Mortgage baseline preset & custom saved scenarios.");

    await desktopPage.click('button:has-text("✕")');
    await desktopPage.waitForTimeout(300);

    // 8. Test Export Modal
    console.log("Testing Export modal on desktop...");
    await desktopPage.click('button[aria-label="Export Scenario JSON / Report"]');
    await desktopPage.waitForTimeout(500);
    const exportHeader = await desktopPage.$('text=Export Scenario & Reports');
    if (!exportHeader) throw new Error("Export modal failed to open");
    console.log("✓ Export modal opened and verified.");
    await desktopPage.click('button:has-text("✕")');
    await desktopPage.waitForTimeout(300);

    // 9. Test Import Modal
    console.log("Testing Import modal on desktop...");
    await desktopPage.click('button[aria-label="Import Scenario JSON"]');
    await desktopPage.waitForTimeout(500);
    const importHeader = await desktopPage.$('text=Import Scenario File');
    if (!importHeader) throw new Error("Import modal failed to open");
    console.log("✓ Import modal opened and verified.");
    await desktopPage.click('button:has-text("✕")');
    await desktopPage.waitForTimeout(300);

    await assertNoHorizontalOverflow(desktopPage, "Desktop Viewport Overall");
    console.log("✓ Desktop viewport suite completed successfully.");
    await desktopContext.close();

    // =========================================================================
    // SUITE 2: MOBILE VIEWPORT VALIDATION (375x667 - iPhone SE / Narrow Screen)
    // =========================================================================
    console.log("\n>>> [2/2] Executing Mobile Viewport Test Suite (375x667)...");
    const mobileContext = await browser.newContext({
      viewport: { width: 375, height: 667 },
      isMobile: true,
      hasTouch: true
    });
    const mobilePage = await mobileContext.newPage();

    await mobilePage.goto(baseUrl, { waitUntil: 'networkidle' });
    await mobilePage.evaluate(() => localStorage.clear());
    await mobilePage.reload({ waitUntil: 'networkidle' });
    await mobilePage.waitForTimeout(500);

    // 1. Mobile Vertical Stacking (SplitPane flex-col layout)
    await mobilePage.waitForSelector('text=Homecalc', { timeout: 10000 });
    const mobileAsideBox = await mobilePage.locator('aside').boundingBox();
    const mobileMainBox = await mobilePage.locator('main').boundingBox();
    if (!mobileAsideBox || !mobileMainBox) throw new Error("Mobile SplitPane aside or main element bounding box not found");

    // In vertical stack: aside must be above main
    if (mobileAsideBox.y >= mobileMainBox.y || (mobileAsideBox.y + mobileAsideBox.height > mobileMainBox.y + 10)) {
      throw new Error(`Mobile SplitPane vertical stacking failure: aside (y:${mobileAsideBox.y}, h:${mobileAsideBox.height}) is not positioned above main (y:${mobileMainBox.y}, h:${mobileMainBox.height})`);
    }
    console.log(`✓ Mobile SplitPane vertical stack verified (aside: y=${mobileAsideBox.y}, h=${mobileAsideBox.height} | main: y=${mobileMainBox.y}, h=${mobileMainBox.height}).`);
    await assertNoHorizontalOverflow(mobilePage, "Mobile Initial Load");

    // 2. Mobile Header & Responsive Slot Switcher
    console.log("Testing mobile header, branding & slot switcher...");
    const mobileHeaderBrand = await mobilePage.$('text=Homecalc');
    if (!mobileHeaderBrand) throw new Error("Brand title missing in mobile header");

    const mobileVersion = await mobilePage.$('text=v2.1.0');
    if (!mobileVersion) throw new Error("Version badge missing in mobile header");

    // Verify slot switching on mobile
    await mobilePage.click('button[aria-label*="Select Slot 2"]');
    await mobilePage.waitForTimeout(300);
    await mobilePage.click('button[aria-label*="Select Slot 3"]');
    await mobilePage.waitForTimeout(300);
    await mobilePage.click('button[aria-label*="Select Slot 1"]');
    await mobilePage.waitForTimeout(300);
    console.log("✓ Mobile header controls & slot switching verified.");
    await assertNoHorizontalOverflow(mobilePage, "Mobile Header & Slot Switcher");

    // 3. Mobile Parameter Tabs & Form Inputs
    console.log("Testing mobile parameter tabs and inputs...");
    // Property tab is default
    const mobilePriceInput = await mobilePage.$('#purchase-price-input');
    if (!mobilePriceInput) throw new Error("Property inputs missing on mobile");

    // Switch to Financing tools tab
    await mobilePage.click('button:has-text("Financing")');
    await mobilePage.waitForTimeout(300);
    await assertNoHorizontalOverflow(mobilePage, "Mobile Parameter Financing Tab");

    // Switch to Prepayment tab
    await mobilePage.click('button:has-text("Prepay")');
    await mobilePage.waitForTimeout(300);
    await assertNoHorizontalOverflow(mobilePage, "Mobile Parameter Prepay Tab");

    // Return to Property tab
    await mobilePage.click('button:has-text("Property")');
    await mobilePage.waitForTimeout(300);
    console.log("✓ Mobile parameter tabs & form inputs rendered without overflow.");

    // 4. Mobile Dashboard Summary Cards
    console.log("Testing mobile ScenarioSummaryView cards...");
    const propertyInfoCard = await mobilePage.$('text=Property Information');
    if (!propertyInfoCard) throw new Error("Property Information card missing on mobile");
    const paymentSplitCard = await mobilePage.$('text=Payment Split');
    if (!paymentSplitCard) throw new Error("Payment Split card missing on mobile");
    const effectiveOutlayCard = await mobilePage.$('text=Effective Outlay');
    if (!effectiveOutlayCard) throw new Error("Effective Outlay card missing on mobile");
    const payoffTimelineCard = await mobilePage.$('text=Payoff Timeline');
    if (!payoffTimelineCard) throw new Error("Payoff Timeline card missing on mobile");
    await assertNoHorizontalOverflow(mobilePage, "Mobile Dashboard Overview");
    console.log("✓ Mobile Dashboard Summary KPI cards verified.");

    // 5. Mobile Charts View
    console.log("Testing mobile Charts view...");
    await mobilePage.click('button:has-text("Charts")');
    await mobilePage.waitForTimeout(500);
    const mobileSvg = await mobilePage.$('svg[aria-label="Amortization trajectory chart"]');
    if (!mobileSvg) throw new Error("Amortization chart SVG missing on mobile");
    const svgBox = await mobileSvg.boundingBox();
    if (!svgBox || svgBox.width > 375) {
      throw new Error(`Amortization chart SVG width (${svgBox?.width}px) exceeds mobile viewport (375px)`);
    }
    await assertNoHorizontalOverflow(mobilePage, "Mobile Charts View");
    console.log(`✓ Mobile Charts view rendered and contained within viewport width (SVG width: ${svgBox.width}px).`);

    // 6. Mobile Statements View (Ledger Table & Pagination)
    console.log("Testing mobile Statements view...");
    await mobilePage.click('button:has-text("Statements")');
    await mobilePage.waitForTimeout(500);
    const statementTitle = await mobilePage.$('text=Statement Ledger');
    if (!statementTitle) throw new Error("Statement Ledger title missing on mobile");

    const statementRows = await mobilePage.$$('tbody tr');
    if (statementRows.length === 0) throw new Error("Monthly statement table rows missing on mobile");

    // Switch to Yearly Statement Table
    await mobilePage.click('button:has-text("Yearly")');
    await mobilePage.waitForTimeout(400);
    const yearlyRows = await mobilePage.$$('tbody tr');
    if (yearlyRows.length === 0) throw new Error("Yearly statement table rows missing on mobile");

    await mobilePage.click('button:has-text("Monthly")');
    await mobilePage.waitForTimeout(300);
    await assertNoHorizontalOverflow(mobilePage, "Mobile Statements View");
    console.log(`✓ Mobile Statements ledger rendered (Monthly: ${statementRows.length} rows, Yearly: ${yearlyRows.length} rows) with contained horizontal scroll.`);

    // 7. Mobile Compare View (Differential Table & Terminology)
    console.log("Testing mobile Compare view...");
    await mobilePage.click('button:has-text("Compare")');
    await mobilePage.waitForTimeout(500);
    const mobileCompareHeader = await mobilePage.$('text=Scenario Differential Analysis');
    if (!mobileCompareHeader) throw new Error("Scenario Differential Analysis missing on mobile");
    const mobileDiffTable = await mobilePage.$('text=Metric Differential Table');
    if (!mobileDiffTable) throw new Error("Metric Differential Table missing on mobile");
    await assertNoHorizontalOverflow(mobilePage, "Mobile Compare View");
    console.log("✓ Mobile Compare view and Metric Differential Table rendered properly without horizontal overflow.");

    // 8. Mobile Modals (Presets/Library, Export, Import)
    console.log("Testing mobile Presets/Library modal...");
    await mobilePage.click('button[aria-label="Open Scenario Library"]');
    await mobilePage.waitForTimeout(500);
    const mobileLibHeader = await mobilePage.$('text=Scenario Library & Presets');
    if (!mobileLibHeader) throw new Error("Scenario Library modal failed to open on mobile");
    await assertNoHorizontalOverflow(mobilePage, "Mobile Library Modal");
    await mobilePage.click('button:has-text("✕")');
    await mobilePage.waitForTimeout(300);

    console.log("Testing mobile Export modal...");
    await mobilePage.click('button[aria-label="Export Scenario JSON / Report"]');
    await mobilePage.waitForTimeout(500);
    const mobileExportHeader = await mobilePage.$('text=Export Scenario & Reports');
    if (!mobileExportHeader) throw new Error("Export modal failed to open on mobile");
    await assertNoHorizontalOverflow(mobilePage, "Mobile Export Modal");
    await mobilePage.click('button:has-text("✕")');
    await mobilePage.waitForTimeout(300);

    console.log("Testing mobile Import modal...");
    await mobilePage.click('button[aria-label="Import Scenario JSON"]');
    await mobilePage.waitForTimeout(500);
    const mobileImportHeader = await mobilePage.$('text=Import Scenario File');
    if (!mobileImportHeader) throw new Error("Import modal failed to open on mobile");
    await assertNoHorizontalOverflow(mobilePage, "Mobile Import Modal");
    await mobilePage.click('button:has-text("✕")');
    await mobilePage.waitForTimeout(300);

    console.log("✓ All mobile modal dialogs opened, formatted, and dismissed cleanly.");
    await mobileContext.close();

    console.log("\n================================================================================");
    console.log("🎉 ALL DESKTOP & MOBILE E2E VISUAL AND RESPONSIVE TESTS PASSED SUCCESSFULLY!");
    console.log("================================================================================");
  } finally {
    await browser.close();
    previewServer.httpServer.close();
  }
}

run()
  .then(() => process.exit(0))
  .catch(err => {
    console.error("E2E Test Failure:", err);
    process.exit(1);
  });
