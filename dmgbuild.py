import os

application = "TPS Cube.app"
app = application
appname = os.path.basename(application)
format = defines.get('format', 'UDBZ')
files = [application]
symlinks = {'Applications': '/Applications'}
badge_icon = "./images/icon.icns"
icon_locations = {appname: (50, 50), 'Applications': (200, 50)}
window_rect = ((100, 100), (328, 228))
default_view = 'icon-view'
show_icon_preview = False
include_icon_view_settings = 'auto'
include_list_view_settings = 'auto'
arrange_by = None
grid_offset = (0, 0)
grid_spacing = 50
scroll_position = (0, 0)
label_pos = 'bottom'
text_size = 16
icon_size = 128
