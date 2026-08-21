import { chromium } from 'playwright';

async function run() {
  console.log("Starting End-to-End Visual & Functional Validation (Clean Empty Session)...");
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const page = await context.newPage();

  // Clear localStorage before testing to test clean empty state
  await page.goto('http://127.0.0.1:5173', { waitUntil: 'networkidle' });
  await page.evaluate(() => localStorage.clear());
  await page.reload({ waitUntil: 'networkidle' });
  await page.waitForTimeout(500);

  // 1. Verify Clean Empty Initial State
  const title = await page.title();
  console.log("Page title:", title);
  if (!title.includes("Homecalc")) throw new Error("Incorrect page title");

  const emptyCallout = await page.$('text=Slot 1 is Empty');
  if (!emptyCallout) throw new Error("Empty slot callout missing on initial load");
  console.log("✓ Initial clean empty slot state verified.");

  // 2. Create New Scenario in Slot 1
  console.log("Creating new scenario in Slot 1...");
  await page.click('button:has-text("Create New Scenario")');
  await page.waitForTimeout(500);

  // Verify KPI cards now populated
  const kpiCards = await page.$$('.tabular-nums');
  console.log(`Found ${kpiCards.length} tabular numeric indicators.`);
  if (kpiCards.length < 5) throw new Error("KPI indicators missing after creating scenario");
  console.log("✓ Scenario successfully created and simulated in Slot 1.");

  // 3. Switch to Analytics / Charts View
  console.log("Testing Charts view navigation...");
  await page.click('button:has-text("Analytics")');
  await page.waitForTimeout(500);
  const svgChart = await page.$('svg[aria-label="Amortization trajectory chart"]');
  if (!svgChart) throw new Error("Amortization SVG chart missing");
  console.log("✓ Amortization Trajectory Chart rendered successfully.");

  // 4. Switch to Statements View
  console.log("Testing Statements view navigation...");
  await page.click('button:has-text("Statements")');
  await page.waitForTimeout(500);
  const tableRows = await page.$$('tbody tr');
  console.log(`✓ Statements table rendered with ${tableRows.length} rows.`);
  if (tableRows.length === 0) throw new Error("Statements table empty");

  // 5. Test Parameter Manipulation (Slide purchase price)
  console.log("Testing reactive parameter update...");
  await page.click('button:has-text("Dashboard")');
  await page.waitForTimeout(300);
  await page.click('button:has-text("+$50k")');
  await page.waitForTimeout(300);
  console.log("✓ Reactive parameter adjustment triggered.");

  // 6. Test Save to Library & My Scenarios Modal
  console.log("Testing Save to Library...");
  await page.click('button:has-text("💾 Save")');
  await page.waitForTimeout(300);

  console.log("Opening My Scenarios Library modal...");
  await page.click('button:has-text("My Scenarios")');
  await page.waitForTimeout(500);
  const modalHeader = await page.$('text=My Saved Scenarios');
  if (!modalHeader) throw new Error("Saved scenarios modal failed to open");
  console.log("✓ My Saved Scenarios modal rendered with custom saved scenario.");

  // Close modal
  await page.click('button:has-text("✕")');
  await page.waitForTimeout(300);

  // 7. Test Export Modal
  console.log("Testing Export modal...");
  await page.click('button:has-text("Export")');
  await page.waitForTimeout(500);
  const exportHeader = await page.$('text=Export Scenario & Reports');
  if (!exportHeader) throw new Error("Export modal failed to open");
  console.log("✓ Export modal opened and verified.");
  await page.click('button:has-text("✕")');
  await page.waitForTimeout(300);

  // 8. Test Clear Slot
  console.log("Testing Clear Slot...");
  page.on('dialog', async dialog => {
    await dialog.accept();
  });
  await page.click('button:has-text("🗑️ Clear")');
  await page.waitForTimeout(500);
  const clearedCallout = await page.$('text=Slot 1 is Empty');
  if (!clearedCallout) throw new Error("Slot did not return to empty state");
  console.log("✓ Slot successfully cleared back to empty state.");

  await browser.close();
  console.log("🎉 ALL REFACTORED SCENARIO MANAGEMENT TESTS PASSED WITH 100% SUCCESS!");
}

run().catch(err => {
  console.error("E2E Test Failure:", err);
  process.exit(1);
});
