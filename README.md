# AutomatedDiscordRichPresence
Create custom Discord Rich Presences that toggle on while a certain app is running.  
This is not a 'serious' project. I am doing this because I've started learning Rust and wanted to write something in it. So don't expect the code to make sense.  
ONLY WORKS ON WINDOWS
# Usage
Compile main.rs and write custom commands inside `config.ini`.  
There is an example `config.ini` file included.  
The following arguments can be used:  
 details  
 state  
 large_image  
 small_image  
 large_text   
 small_text  
'what_to_detect' can be used as a single string or as a few strings that must or must not be included in the window's name.
It can be used like this `[firefox.exe, -private]`.
Here the code will look for a window that includes 'firefox.exe' but does NOT include 'private'.  
This is not case sensitive. 
If you want to make sure the string '-private' is included in the app window write `[firefox.exe, \-private]`.  
To leave the place as a placeholder write `something = ..`.  
If details is left as `..` the full name of the window will be used.  
If state is left as `..` it will be used to fit a part of the description if the description is longer than 20 chars.  
If the images are left as placeholders they will not be used.  
Image text left as placeholder will also change to the full name of the window.  
# Notes
Discord lets you use up to 150 images currently only 34 are being used  
I will probably add more images in the future  
# Images that can be used right now
androidstudiologo  
chromelogo  
cpplogo  
crunchyrolllogo  
csharplogo  
discordlogo  
eclipselogo  
excellogo  
firefoxlogo  
gimplogo  
githublogo  
intellijidealogo  
javalogo  
javascriptlogo  
libreofficelogo  
mangadexlogo  
netflixlogo  
photoshoplogo  
powerpointlogo  
premiereprologo  
pycharmlogo  
pythonlogo  
readthedocslogo  
redditlogo  
rustlogo  
spotifylogo  
stackoverflowlogo  
twitchlogo  
twitterlogo  
visualstudiocodelogo  
visualstudiologo  
wikipedialogo  
wordlogo  
youtubelogo  
