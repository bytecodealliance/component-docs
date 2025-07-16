(module
   (func $add (param $lhs i32) (param $rhs i32) (result i32)
       local.get $lhs
       local.get $rhs
       i32.add)
    (export "docs:adder/add@0.1.0#add" (func $add))
)
