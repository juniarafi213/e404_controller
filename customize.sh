# E404 Controller - Module Installation Script
# Compatible with KernelSU, Magisk, and APatch

SKIPUNZIP=1

ui_print "*********************************************************"
ui_print "          E404 CONTROLLER - SM8150 OPTIMIZER             "
ui_print "         Tuned for Poco X3 Pro & 4.19.404R BORE          "
ui_print "*********************************************************"

# Verify 64-bit ARM
if [ "$ARCH" != "arm64" ]; then
    ui_print "! Unsupported architecture: $ARCH (Requires arm64)"
    abort "*********************************************************"
fi

ui_print "- Extracting module files..."
unzip -o "$ZIPFILE" -x 'META-INF/*' -d "$MODPATH" >&2

# Check Kernel Features
ui_print "- Detecting hardware and kernel features:"

if [ -f "/proc/sys/kernel/sched_bore" ]; then
    ui_print "  [✓] BORE 7.0.1 Scheduler detected"
else
    ui_print "  [!] BORE scheduler node not found (using standard CFS fallback)"
fi

if [ -d "/sys/class/kgsl/kgsl-3d0" ]; then
    opp_count=$(cat /sys/class/kgsl/kgsl-3d0/num_pwrlevels 2>/dev/null || echo 0)
    ui_print "  [✓] Adreno 640 GPU detected ($opp_count OPP power levels)"
fi

if [ -f "/sys/class/power_supply/battery/input_suspend" ]; then
    ui_print "  [✓] Bypass Charging supported (/sys/class/power_supply/battery/input_suspend)"
fi

if [ -f "/sys/kernel/fast_charge/force_fast_charge" ]; then
    ui_print "  [✓] 6A Fast Charge Override supported"
fi

if [ -d "/sys/kernel/e404/lyb" ]; then
    ui_print "  [✓] Lyb Touch Game Mode driver detected"
fi

if [ -f "$MODPATH/bin/system_monitor.apk" ]; then
    ui_print "  [✓] Auto-Game Monitor Daemon engine included"
fi

# Set executable permissions
ui_print "- Setting file permissions..."
set_perm_recursive "$MODPATH" 0 0 0755 0644
set_perm "$MODPATH/bin/e404-ctrl" 0 0 0755
set_perm "$MODPATH/bin/e404-daemon" 0 0 0755
set_perm "$MODPATH/service.sh" 0 0 0755
set_perm "$MODPATH/action.sh" 0 0 0755
set_perm "$MODPATH/webroot/cgi-bin/api" 0 0 0755

# Symlink CLI
if [ -d "/data/adb/ksu/bin" ]; then
    ln -sf "$MODPATH/bin/e404-ctrl" /data/adb/ksu/bin/e404-ctrl
    ln -sf "$MODPATH/bin/e404-daemon" /data/adb/ksu/bin/e404-daemon
elif [ -d "/data/adb/magisk" ]; then
    ln -sf "$MODPATH/bin/e404-ctrl" /data/adb/magisk/e404-ctrl
    ln -sf "$MODPATH/bin/e404-daemon" /data/adb/magisk/e404-daemon
fi

# Initialize default configs if none exists
mkdir -p "$MODPATH/config"
[ ! -f "$MODPATH/config/current_profile" ] && echo "balanced" > "$MODPATH/config/current_profile"
[ ! -f "$MODPATH/config/e404.conf" ] && cat <<EOF > "$MODPATH/config/e404.conf"
PERF_LITE=1
AUTO_GAME=1
BYPASS=0
FASTCHARGE=1
LYB=0
EOF

ui_print ""
ui_print "  Installation completed successfully!"
ui_print "  - WebUI: Open module in KernelSU Manager or http://127.0.0.1:8404"
ui_print "  - CLI Tool: Type 'su -c e404-ctrl' in terminal"
ui_print "*********************************************************"
