struct Opt <T: type>
{
    data: T,
    is: bool,
}

struct Vector
{
    x: f32,
    y: f32,
    z: f32,
}

struct Buffer <T: type>: Destroy, Copy
{
    size: usize,
    capa: usize,
    data: &T,
}

struct System
{
    positions: Buffer<Vector>,
    colors: Buffer<Vector>,
}

impl Buffer
{
    fn new() -> Buffer
    {
    }

    fn .destroy(self) -> void
    {
    }

    fn .copy(self) -> Buffer
    {
    }
}

let v1 = Buffer::new();
let v2 = smove(v1);

