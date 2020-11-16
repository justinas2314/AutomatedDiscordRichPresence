# AutomatedDiscordRichPresence
Create custom Discord Rich Presences that toggle on while a certain app is running.  
# Disclaimers
This is not a 'serious' project. I am doing this because I've started learning Rust and wanted to write something in it. So don't expect the code to make sense.  
Only works on Windows.  
Non ASCII Strings are buggy because the code needs to communicate with windows using commands and windows does not (atleast for me) output the chars correctly.  
# Usage
Compile main.rs and write custom commands inside `config.ini`.  
There is an example `config.ini` file included.  
If you are going to run the .exe file move the config forder to the target (where the .exe file is) directory.  
If you are going to run via `cargo run --release` you don't need to move the file.  
The following arguments can be used:  
 details  
 state  
 large_image  
 small_image  
 large_text   
 small_text  
 + regex
The first line is where the preset will be defined. It can be written like this `[firefox.exe, -private]`.
Here the code will look for a window that includes 'firefox.exe' but does NOT include 'private'.  
This is not case sensitive. 
If you want to make sure the string '-private' is included in the app window write `[firefox.exe, \-private]`.  
To leave the place as a placeholder write `something = ..`.  
If details is left as `..` the full name of the window will be used.  
If state is left as `..` it will be used to fit a part of the description if the description is longer than 20 chars.  
If the images are left as placeholders they will not be used.  
Image text left as placeholder will also change to the full name of the window.  
We can also inherit placeholders.  
If we define a template as `![TEMPLATE]`  
`state = this is a state`  
and then pass it to a different preset `[preset] <- [TEMPLATE]` we will not have to define the state again.  
This is useful for setting the same value in multiple presets without writing too much.  
We can also define the template as `[TEMPLATE]` but the template would then match all windows that include 'template'.  
'!' excludes the template from getting matched to window titles.
# How to use regex
You do not need to use regex but your discord status will be more boring :].  
details_, state_, large_ and small_ can be used to set up regex for details, state, large_text, small_text  
there are 3 arguments:  
 * regex - this regex will be applied to the window name of the window that was matched. This will split the text into groups.
 * format - this is a string used to put the groups into one piece. Example: `{2} and {1}`. The number inside `{}` is the group's index (indexing starts from 1, 0 is the entire matched regex). If group 2 matched to `two` and group 1 matched to `one` the final string will be `two and one`.
 * fallback (optional) - if nothing gets inserted into `format` this will be the string shown instead.
So you could set the large_text field with regex by writing something like this:  
```
large_regex = (\\S*) - (\\S*)
large_format = working on a file '{2}' in my project '{1}'
large_fallback = working on something cool
```
If the window's name is `Project - main.rs - IntelliJ IDEA` group 1 will be `Project` and group 2 will be `main.rs`  
So the text on the large image will be `working on a file 'main.rs' in my project 'Project'`.  
And if the window's name is `Minecraft` `large_regex` will not find anything so nothing will be inserted into `large_format`  
and `large_fallback` will be used and the text on the large image will be `working on something cool`.
# Notes
Discord lets you use up to 150 images currently only 53 are being used  
I will probably add more images in the future  
# Images that can be used right now
androidstudiologo  
arduinologo  
blenderlogo  
chromelogo  
clogo  
cpplogo  
crunchyrolllogo  
csharplogo  
curiouscatlogo  
discordlogo  
eclipselogo  
excellogo  
facebooklogo  
firefoxlogo  
gimplogo  
githublogo  
gmaillogo  
hululogo  
instagramlogo  
intellijidealogo  
ituneslogo  
javalogo  
javascriptlogo  
libreofficelogo  
linkedinlogo  
mangadexlogo  
messengerlogo  
netflixlogo  
photoshoplogo  
powerpointlogo  
premiereprologo  
pycharmlogo  
pythonlogo  
readthedocslogo  
redditlogo  
rustlogo  
skypelogo  
slacklogo  
snapchatlogo  
spotifylogo  
stackoverflowlogo  
teamspeaklogo  
telegramlogo  
twitchlogo  
twitterlogo  
visualstudiocodelogo  
visualstudiologo  
vlclogo  
whatsapplogo  
wikipedialogo  
wordlogo  
youtubelogo  
zoomlogo  
