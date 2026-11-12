#!/system/bin/sh
# E404 Controller - Module Uninstallation Script

# Kill local web server
pkill -f "8404" 2>/dev/null || true

# Remove CLI symlinks
rm -f /data/adb/ksu/bin/e404-ctrl
rm -f /data/adb/magisk/e404-ctrl

# Restore safe kernel defaults
echo 0 > /sys/class/power_supply/battery/input_suspend 2>/dev/null || true
echo 0 > /sys/class/kgsl/kgsl-3d0/max_pwrlevel 2>/dev/null || true
echo 7 > /sys/class/kgsl/kgsl-3d0/min_pwrlevel 2>/dev/null || true
echo 0 > /sys/kernel/e404/lyb/touch_game_mode 2>/dev/null || true
echo 0 > /sys/devices/virtual/thermal/thermal_message/sconfig 2>/dev/null || true
echo 0 > /sys/devices/virtual/thermal/thermal_message/balance_mode 2>/dev/null || true

exit 0
