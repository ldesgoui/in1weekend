#[macro_export]
macro_rules! mkScene {
    {
        objects: $objects:tt,
    } => {
        mkScene! {
            background: [
                LinSrgb::new(0.4, 0.5, 1.0),
                LinSrgb::new(1.0, 1.0, 1.0),
                LinSrgb::new(0.4, 0.5, 1.0)
            ],
            objects: $objects,
        }
    };

    {
        background: [ $( $color:expr ),+ ],
        objects: [ $( $object:tt ),+ ],
    } => {
        use nc::shape::Shape;

        Scene {
            background: palette::gradient::Gradient::new(vec![ $( $color ),* ]),
            objects: std::sync::Arc::new(
                BVT::new_balanced(vec![ $( mkObject!($object) ),+ ]),
            ),
        }
    };
}

macro_rules! mkObject {
    {{
        shape: $shape:expr,
        material: $material:expr,
    }} => {
        mkObject!({
            shape: $shape,
            material: $material,
            transform: Isometry::identity(),
        })
    };

    {{
        shape: $shape:expr,
        material: $material:expr,
        rotation: $rotation:expr,
    }} => {
        mkObject!({
            shape: $shape,
            material: $material,
            transform: Isometry::new(Vector::zeros(), $rotation),
        })
    };

    {{
        shape: $shape:expr,
        material: $material:expr,
        translation: $translation:expr,
    }} => {
        mkObject!({
            shape: $shape,
            material: $material,
            transform: Isometry::new($translation, Vector::zeros()),
        })
    };

    {{
        shape: $shape:expr,
        material: $material:expr,
        translation: $translation:expr,
        rotation: $rotation:expr,
    }} => {
        mkObject!({
            shape: $shape,
            material: $material,
            transform: Isometry::new($translation, $rotation),
        })
    };

    {{
        shape: $shape:expr,
        material: $material:expr,
        transform: $transform:expr,
    }} => {
        (
            Box::new(crate::object::ObjectInner {
                shape: $shape,
                material: $material,
                transform: $transform,
            }),
            $shape.aabb(&$transform),
        )
    };
}
