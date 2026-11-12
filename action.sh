#!/system/bin/sh
# E404 Controller - Manager Action Script

MODDIR="$(dirname "$0")"

# Ensure server is running
BUSYBOX="/data/adb/ksu/bin/busybox"
[ ! -x "$BUSYBOX" ] && BUSYBOX="/data/adb/magisk/busybox"
if [ -x "$BUSYBOX" ] && ! pgrep -f "8404" >/dev/null; then
    "$BUSYBOX" httpd -p 127.0.0.1:8404 -h "$MODDIR/webroot"
fi

echo "========================================"
echo "         E404 CONTROLLER (SM8150)       "
echo "========================================"

if [ -f "$MODDIR/bin/e404-ctrl" ]; then
    cur_prof=$("$MODDIR/bin/e404-ctrl" profile get 2>/dev/null || echo "balanced")
    echo "Current Profile: $(echo "$cur_prof" | tr 'a-z' 'A-Z')"
fi

echo ""
echo "Launching WebUI..."

# Check WebUIStandalone
if pm path io.github.a13e300.ksuwebui >/dev/null 2>&1; then
    am start -n "io.github.a13e300.ksuwebui/.WebUIActivity" -e id "e404_controller" >/dev/null 2>&1
    echo "Opened via KsuWebUIStandalone."
    exit 0
fi

# Check WebUI X
if pm path com.dergoogler.mmrl.wx >/dev/null 2>&1; then
    am start -n "com.dergoogler.mmrl.wx/.ui.activity.webui.WebUIActivity" -e MOD_ID "e404_controller" >/dev/null 2>&1
    echo "Opened via WebUI X."
    exit 0
fi

# Fallback: Launch in Android Default Browser via localhost
am start -a android.intent.action.VIEW -d "http://127.0.0.1:8404" >/dev/null 2>&1
echo "Opened in browser: http://127.0.0.1:8404"
exit 0
