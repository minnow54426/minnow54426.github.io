AUTHOR = 'cryptboy'
SITENAME = 'wonderonpathlesspath'
SITEURL = ""

PATH = "content"

TIMEZONE = 'Asia/Shanghai'

DEFAULT_LANG = 'en'

# Feed generation is usually not desired when developing
FEED_ALL_ATOM = None
CATEGORY_FEED_ATOM = None
TRANSLATION_FEED_ATOM = None
AUTHOR_FEED_ATOM = None
AUTHOR_FEED_RSS = None

# Blogroll
# LINKS = (
#     ("Pelican", "https://getpelican.com/"),
#     ("Python.org", "https://www.python.org/"),
#     ("Jinja2", "https://palletsprojects.com/p/jinja/"),
#     ("You can modify those links in your config file", "#"),
# )

# Social widget
# SOCIAL = (
#     ("You can add links in your config file", "#"),
#     ("Another social link", "#"),
# )

# Disable footer sections
LINKS = ()
SOCIAL = ()

DEFAULT_PAGINATION = False

# Uncomment following line if you want document-relative URLs when developing
# RELATIVE_URLS = True

# Category settings for Photography, Code, and Music
USE_FOLDER_AS_CATEGORY = True
DEFAULT_CATEGORY = 'misc'
CATEGORY_URL = 'category/{slug}.html'
CATEGORY_SAVE_AS = 'category/{slug}.html'

# Static files (images, audio, photos)
STATIC_PATHS = ['images', 'audio', 'photos', 'photos/organized', 'photography']

# Theme settings
THEME = "theme"

# Plugin settings
PLUGIN_PATHS = ['plugins']
PLUGINS = ['photo_gallery']
