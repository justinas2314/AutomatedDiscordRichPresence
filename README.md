# AutomatedDiscordRichPresence
While the code runs it updates the Discord rich presence
# Usage
Compile main.rs and write custom commands inside `commands.txt`.  
The commands look like this:  
what_to_detect,, description,, state,, bigimage,, smallimage,, bigimagetext,, smallimagetext.  
'what_to_detect' can be used as a single string or as a few strings that must or must not be included in the window's name.
It can be used like this `[firefox.exe,,--private]` or like this `firefox.exe`.
Notice how it has to be `[firefox.exe,,--private]` and not `[firefox.exe,, --private]`.
The second one searches for a string that includes '--private'. The code searches for the pattern `,,--`.
The first one will match the app 'firefox.exe' but only if 'private' is not found in the name of the window.
The second will always match 'firefox.exe'.  
THERE IS NO ESCAPE for `,,`. You cannot escape ',,' it will always be used to seperate values.  
You can escape the brackets by writing `\[` and `\]` but you cannot escape the escape characters themselves.  
To leave the place blank write `,, ,,`.
DO NOT write `,,,,` it is not clear and doesn't work the same way as `,, ,,` (it bugs out and i'm not fixing it because it is not easily readible. Now you don't have a choice :).  
To leave the place as a placeholder write `,, .. ,,`.  
If description is left as `..` the full name of the window will be used.  
If state is left as `..` it will be used to fit a part of the description if the description is longer than 20 chars.  
If the images are left as placeholders they will not be used.  
Image text left as placeholder will also change to the full name of the window.  
# Images that can be used
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
