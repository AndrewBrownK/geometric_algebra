// Thanks Bing Copilot
/*
Yes, your understanding is correct. In the context of shaders and graphics programming, a "camera"
typically refers to a set of parameters that define the view and projection transformations. These
parameters are often passed to the vertex shader as uniforms because they remain constant for all
vertices during a draw call.

The view position (or camera position) represents the
location of the camera in world space. It's used to transform vertices from world space to view
space (also known as camera space or eye space), which is the space as seen from the camera's
perspective.

The projection matrix is used to project the 3D scene
onto a 2D plane, which is what we see on the screen. This matrix transforms vertices from view
space to clip space. There are two common types of projection:

•  Perspective projection: This mimics how our eyes
perceive the world, with objects appearing smaller as they are farther away. It creates a frustum
where only the objects within a certain range are visible and scaled appropriately.

•  Orthographic projection: This type of projection
maintains the size of objects regardless of their distance from the camera, which is useful for 2D
games and certain types of 3D games where perspective distortion is not desired.

At a high level, the camera projection operation works by taking a point in 3D space and applying
a series of transformations to it so that it can be represented correctly on a 2D screen. Here's a
simplified explanation without delving into matrix math:

World space to View space: First, the vertex's
position in world space is transformed to view space using the view matrix. This step involves
translating and rotating the world so that the camera appears to be at the origin and looking down
the negative z-axis.

View space to Clip space:
Next, the view space coordinates are transformed to clip space using the projection matrix. For
perspective projection, this step involves scaling the x and y coordinates of the vertex relative
to its z coordinate, simulating the effect of depth. For orthographic projection, this step
involves scaling the coordinates uniformly without considering depth.

Clip space to Normalized Device Coordinates (NDC):
After the vertex is in clip space, perspective division is performed by dividing the x, y, and z
coordinates by the w coordinate. This step maps the coordinates to a cube where the range for x, y,
and z is from -1 to 1, known as the NDC.

Viewport Transformation: Finally, the NDC coordinates
are transformed to screen space coordinates based on the viewport's size and position, resulting in
the final position where the vertex will be rendered on the screen.

This sequence of transformations allows a 3D scene to be rendered on a 2D display in a way that
preserves the spatial relationships and proportions of objects, giving the illusion of depth. The
use of homogeneous coordinates and the w component is crucial in these transformations, as it
enables the perspective division and the representation of points at infinity.
 */


fn main() {
    println!("Hello, world!");
}
