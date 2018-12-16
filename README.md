# Wallpaper Classifier
    usage: Image classifier [-help] [-s] [-v | -l] [-w W | -wg WG | -wl WL]
                            [-h H | -hg HG | -hl HL] [-mv MV | -cp CP | -rm]
                            [input_path]
    
    positional arguments:
      input_path  Path (folder) of images
    
    optional arguments:
      -help       show this help message and exit
    
    Shape Option:
      Select Square/Vertical/Horizontal image. Option -s can be used with -v/-l
    
      -s          Square
      -v          Vertical
      -l          Landscape (horizontal)
    
    Width Option:
      -w W        Width is equal to W
      -wg WG      Width is equal to or greater than WG
      -wl WL      Width is equal to or less than WL
    
    Height Option:
      -h H        Height is equal to H
      -hg HG      Height is equal to or greater than HG
      -hl HL      Height is equal to or greater than HL
    
    Process for matching image(s):
      -mv MV      Move matched images to MV
      -cp CP      Copy matched images to CP
      -rm         Remove matched images
