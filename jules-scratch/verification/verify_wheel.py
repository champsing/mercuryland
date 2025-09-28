from playwright.sync_api import sync_playwright, expect

def run(playwright):
    browser = playwright.chromium.launch()
    page = browser.new_page()
    page.goto("http://localhost:5173/wheel")

    # Wait for the page to load and the text areas to be present
    expect(page.get_by_text("待抽區")).to_be_visible()

    # Add items to the wheel
    text_area = page.locator('textarea').first
    text_area.press_sequentially("Item 1\nItem 2\nItem 3\nItem 4x2")

    # Take a screenshot before spinning
    page.screenshot(path="jules-scratch/verification/wheel_before_spin.png")

    # Wait for the spin button to be enabled, then click it
    spin_button = page.get_by_role("button", name="旋轉")
    expect(spin_button).to_be_enabled()
    spin_button.click()

    # Wait for the spin to finish and the modal to appear
    # The modal has a button with the text "移动"
    expect(page.get_by_role("button", name="移动")).to_be_visible(timeout=15000)

    # Take a screenshot of the result
    page.screenshot(path="jules-scratch/verification/verification.png")

    browser.close()

with sync_playwright() as playwright:
    run(playwright)