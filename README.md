rWifi Rust TUI Wi-Fi Station Manager

rWifi is a lightweight, responsive, and premium terminal user interface (TUI) custom-tailored for Windows to view, switch, and select Wi-Fi networks. It uses native Win32 WLAN APIs for maximum speed and efficiency.


Key Features

Real-time Wi-Fi Scanner: Scans and displays available Wi-Fi networks dynamically, detailing SSID, signal strength, security algorithm, cipher type, and profile status.

Seamless Connections: Connects to networks instantly. For secured networks without saved profiles, a clean password input overlay text box is presented.

Profile Management: Delete stored profiles or disconnect from the active wireless network directly within the interface.

Auto-Scaling Accent Theme: Integrates with the Windows registry to automatically fetch DWM accent colors AccentColor and theme mode configurations AppsUseLightTheme to paint focus borders and title bars.

Console Tab Title Guard: Sets the console tab title to rWif on launch and restores the original terminal tab title cleanly on exit.

Borderless TUI Frame: Strips default console frame borders and centers the window layout dynamically based on active screen resolution and DPI settings.

Keyboard and Mouse Focus: Supports scroll wheel events for modals and click-and-drag mouse text highlights that automatically copy selected terminal cells to the clipboard upon release.

Power-Aware Throttling: Automatically detects battery status to double the TUI refresh poll rate (conserving CPU cycles and extending battery life).


Keyboard Controls

Tab: Cycle focus between the Wi-Fi Station List and the Connection Info Panel.
Up / Down or k / j: Scroll through available Wi-Fi networks.
Space / r: Manually trigger a new Wi-Fi station scan.
Enter: Connect to the selected Wi-Fi network.
d / Delete: Disconnect from the current network or delete the selected profile from Windows.
h: Toggle the keyboard shortcuts overlay modal.
w / W: Connect to a hidden network.
F1 through F7: Open scrollable Markdown Document Viewer modal:
F1 opens README.md
F2 opens SUPPORT.md
F3 opens LICENSE.md
F4 opens COPYRIGHT.md
F5 opens PRIVACY.md
F6 opens SECURITY.md
F7 opens CONTRIBUTING.md
Mouse Left Click and Drag: Highlight custom text anywhere on the screen.
Mouse Left Click Release: Automatically copy the highlighted text to the Windows Clipboard.
Mouse Scroll Wheel: Scroll up/down in active document or help modals.
q / Esc: Close document view, cancel password entry overlay, or cleanly exit the application.


System Requirements and Integration

OS: Windows 10 / Windows 11 (64-bit).
Native WLAN Service: Relies on the standard Windows WLAN AutoConfig Service WlanSvc.
Console Layout Grid: Requires a console window size of at least 110x38 cells. If the terminal size drops below this, a layout constraints guard will display a centering layout warning.


Local Configuration

All configuration details are stored in your Windows user profile at:
%APPDATA%\rWifi\config.yaml

Available options in config.yaml:
theme_mode: auto (default, matches Windows Theme), dark, or light.
refresh_rate_ms: Poll rate interval in milliseconds (default: 100).
enable_borderless: true to strip console headers and frame borders.
enable_toasts: true to trigger native desktop toast alerts.
enable_event_log: true to sync application logs to the Windows Event Viewer.


Build Instructions

Ensure you have the Rust compiler toolchain installed.

Clone or copy the source code directory.

Compile the release binary:
cargo build --release

The executable rwif.exe will be located under target\release\. The compilation script embeds the application icon resources automatically into the final executable.
