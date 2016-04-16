# Cartoon Upsample
Team: Patrick Lorio

The idea of the project is to take a low resolution cartoon video and render a sharp high resolution version. Modern cartoons are generated from vector graphics. The algorithm in this project will try to exploit this fact. For regular films it is hard beat bilinear interpolation. With computer animated videos I think we can do better.

I plan to use ffmpeg to generate images for each frame of the video source. From there my program will take each image and process a higher quality version. The resampled frames will then be stitched back together using ffmpeg to produce the final video.
For the demo I plan on contrasting a 480p and 720p video clip to the 1080p processed version.


#### Schedule (units in weeks)
0.0 - 0.5
 - Use ffmpeg to generate images from source video
 - Split image into quad tree till variance of pixel color is below a defined threshold
 - Step through perimeter of tree’s leafs and collect the leaf’s perimeter as a list of points

0.5 - 1.0
 - Estimate bezier curves to describe perimeter
 - Line up bezier curves between neighboring shapes

1.0 - 1.5
 - Generate svg image for shapes
 - Render with svg rendering engine and tweak system for better performance

1.5 - 2.5
 - Implement rasterization for svg or buffer room for above
 - Use ffmpeg to build video from images

Ideally I would like my algorithm to account for previous frames so that the underlying vector structure can be manipulated to fit the current frame rather than recomputed. This would allow for faster processing and possibly fix probable issues with continuity between frames.
