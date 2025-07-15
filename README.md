<!-- vale off -->
# WF-C

## Idea
Idea is to build a small representation of how wf-c works by showing a repeatable refreshable render
based on a template we build with the clicker at the start.

## Implementation
Currently looking to use piston to render, look at writing own wf-c implementation to run the bts

## Looks
Little window with square tiles, 512x512 window with 32px tiles, set of six colours, just to
illustrate.

## TODO:
 - Come up with a way to build examples for the seeds. 
    > NEXT TASK: Store tile grid to JSON
    > AFTER: use to imput into wfc function.

 - Implement wf-c logic behind the scenes in order to get single example working
    > Once we have way to build input images, need a function to analyse them.
    > This function can then build weights and rules for wfc

 MIGHT AS WELL:
 - Come up with easy interface to refresh render, swap between seeds.
 - 
