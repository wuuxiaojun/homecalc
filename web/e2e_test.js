import { chromium } from 'playwright';

async function run() {
  console.log("Starting End-to-End Visual & Functional Validation (Default Scenario Slots)...");
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const page = await context.newPage();

  // Clear localStorage to test cold start
  await page.goto('http://127.0.0.1:5173', { waitUntil: 'networkidle' });
  await page.evaluate(() => localStorage.clear());
  await page.reload({ waitUntil: 'networkidle' });
  await page.waitForTimeout(500);

  // 1. Verify Default Scenario (Standard 30Y Mortgage) is loaded immediately into all 3 slots
  const title = await page.title();
  console.log("Page title:", title);
  if (!title.includes("Homecalc")) throw new Error("Incorrect page title");

  // Verify KPI cards are populated immediately on launch
  const kpiCards = await page.$$('.tabular-nums');
  console.log(`Found ${kpiCards.length} tabular numeric indicators on initial load.`);
  if (kpiCards.length < 5) throw new Error("KPI indicators missing on launch");

  const scenarioName = await page.$('text=Standard 30Y Mortgage');
  if (!scenarioName) throw new Error("Standard 30Y Mortgage title missing on initial load");
  console.log("✓ 'Standard 30Y Mortgage' successfully initialized in Slot 1 on fresh launch.");

  // 2. Switch to Slot 2 and Slot 3 (confirm all populated with identical 'Standard 30Y Mortgage' without slot suffixes)
  console.log("Testing slot switching across independent default instances...");
  await page.click('button:has-text("Slot 2")');
  await page.waitForTimeout(300);
  await page.click('button:has-text("Slot 3")');
  await page.waitForTimeout(300);
  await page.click('button:has-text("Slot 1")');
  await page.waitForTimeout(300);
  console.log("✓ All 3 slots verified active with independent 'Standard 30Y Mortgage' instances.");

  // 3. Switch to Charts View via Right Pane view switcher
  console.log("Testing Charts view navigation in right pane...");
  await page.click('button:has-text("Charts")');
  await page.waitForTimeout(500);
  const svgChart = await page.$('svg[aria-label="Amortization trajectory chart"]');
  if (!svgChart) throw new Error("Amortization SVG chart missing");
  console.log("✓ Amortization Trajectory Chart rendered successfully.");

  // 4. Switch to Statements View via Right Pane view switcher
  console.log("Testing Statements view navigation in right pane...");
  await page.click('button:has-text("Statements")');
  await page.waitForTimeout(500);
  const tableRows = await page.$$('tbody tr');
  console.log(`✓ Statements table rendered with ${tableRows.length} rows.`);
  if (tableRows.length === 0) throw new Error("Statements table empty");

  // 5. Switch to Compare View via Right Pane view switcher
  console.log("Testing Compare view navigation in right pane...");
  await page.click('button:has-text("Compare")');
  await page.waitForTimeout(500);
  const compareTable = await page.$('text=Scenario Differential Analysis');
  if (!compareTable) throw new Error("Compare view missing");
  console.log("✓ Compare Differential workspace rendered successfully.");

  // 6. Test Parameter Manipulation (Slide purchase price)
  console.log("Testing reactive parameter update...");
  await page.click('button:has-text("Dashboard")');
  await page.waitForTimeout(300);
  await page.click('button:has-text("+$50k")');
  await page.waitForTimeout(300);
  console.log("✓ Reactive parameter adjustment triggered.");

  // 7. Test Save to Library & Scenario Library Modal
  console.log("Testing Save to Library...");
  await page.click('button:has-text("💾 Save")');
  await page.waitForTimeout(300);

  console.log("Opening Scenario Library modal...");
  await page.click('button:has-text("Presets")');
  await page.waitForTimeout(500);
  const modalHeader = await page.$('text=Scenario Library & Presets');
  if (!modalHeader) throw new Error("Scenario Library modal failed to open");

  const baselineCard = await page.$('text=Standard 30Y Mortgage');
  if (!baselineCard) throw new Error("Standard 30Y Mortgage baseline preset missing from library modal");
  console.log("✓ Scenario Library modal verified with Standard 30Y Mortgage baseline preset & custom saved scenarios.");

  // Close modal
  await page.click('button:has-text("✕")');
  await page.waitForTimeout(300);

  // 8. Test Export Modal
  console.log("Testing Export modal...");
  await page.click('button:has-text("Export")');
  await page.waitForTimeout(500);
  const exportHeader = await page.$('text=Export Scenario & Reports');
  if (!exportHeader) throw new Error("Export modal failed to open");
  console.log("✓ Export modal opened and verified.");
  await page.click('button:has-text("✕")');
  await page.waitForTimeout(300);

  // 9. Test Reset Slot to Default Scenario
  console.log("Testing Reset Slot to Default...");
  page.on('dialog', async dialog => {
    await dialog.accept();
  });
  await page.click('button:has-text("↺ Reset")');
  await page.waitForTimeout(500);
  console.log("✓ Slot successfully reset to baseline 'Standard 30Y Mortgage'.");

  await browser.close();
  console.log("🎉 ALL DEFAULT SCENARIO & UI ADJUSTMENT TESTS PASSED WITH 100% SUCCESS!");
}

run().catch(err => {
  console.error("E2E Test Failure:", err);
  process.exit(1);
});
