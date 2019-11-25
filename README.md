# Alacritty Theme

A simple command line tool to report on and toggle between different color schemes for the terminal emulator [Alacritty](https://github.com/jwilm/alacritty).

[See here](./test/alacritty.yml) for an example of the how you can change your `alacritty.yml` configuration file to allow for toggling between different colors schemes. Most of the configuration file has been omitted, only the relevant parts remain where there is the addition of a new key `color_schemes` that is a mapping to different color schemes you want to have available. Node anchors on each scheme allow referencing these schemes by referencing them in the `colors` attribute.

```yaml
# Here's a simplified example

color_schemes:
  my_theme: &my_theme
    # Here would be a mapping that Alacritty expects to find in
    # the `colors` attribute, like the one below.

colors: *my_theme
```

## For Vim Users

I'd also like to share how I use this with Vim to align the background of my Vim instance with the background I'm using in Alacritty. It's actually really nice.

For reference, I'm using the [Gruvbox](https://github.com/morhetz/gruvbox) color scheme in Vim, which works quite well with light and dark backgrounds.

```vimscript
" This function will set Vim's background to "light" or "dark"
" depending on if the current color scheme Alacritty is using
" has those keywords in its name.
function! AlignBackground()
  let &background = ( system('alacritty-theme current') =~ "light" ? "light" : "dark" )
  hi Normal guibg=NONE ctermbg=NONE
endfunc

" This function will toggle Alacritty's color scheme back and
" forth between light and super_dark themes. You can find them
" in their entirety in `test/alacritty.yml` in this repository.
function! ToggleAlacrittyTheme()
  if (system('alacritty-theme current') =~ "light")
    call system('alacritty-theme change gruvbox_super_dark')
  else
    call system('alacritty-theme change gruvbox_light')
  endif
  call AlignBackground()
endfunc

nmap <leader>l :call ToggleAlacrittyTheme()<cr>

call AlignBackground()
```