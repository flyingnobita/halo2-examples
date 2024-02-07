// // We will add instance columns to our lec1 code to have public inputs

// use halo2_proofs::{
//     arithmetic::FieldExt, circuit::*, dev::MockProver, pasta::Fp, plonk::*, poly::Rotation,
// };

// #[derive(Clone, Debug)]
// struct ACell<F: FieldExt>(AssignedCell<F, F>);

// // Defines the configuration of all the columns, and all of the column definitions
// // Will be incrementally populated and passed around
// #[derive(Clone, Debug)]
// struct FibonacciConfig {
//     pub advice: [Column<Advice>; 3],
//     pub selector: Selector,
//     pub instance: [Column<Instance>; 1],
// }

// struct FibonacciChip<F: FieldExt> {
//     config: FibonacciConfig,
//     _marker: std::marker::PhantomData<F>,
//     // In rust, when you have a struct that is generic over a type parameter (here F),
//     // but the type parameter is not referenced in a field of the struct,
//     // you have to use PhantomData to virtually reference the type parameter,
//     // so that the compiler can track it.  Otherwise it would give an error. - Jason
// }

// impl<F: FieldExt> FibonacciChip<F> {
//     // Default constructor
//     fn construct(config: FibonacciConfig) -> Self {
//         Self {
//             config,
//             _marker: std::marker::PhantomData,
//         }
//     }

//     // Configure will set what type of columns things are, enable equality, create gates, and return a config with all the gates
//     fn configure(
//         meta: &mut ConstraintSystem<F>,
//         advice: [Column<Advice>; 3],
//         instance: [Column<Instance>; 1],
//     ) -> FibonacciConfig {
//         let col_a = advice[0];
//         let col_b = advice[1];
//         let col_c = advice[2];
//         let selector: Selector = meta.selector();

//         // enable_equality has some cost, so we only want to define it on rows where we need copy constraints
//         meta.enable_equality(col_a);
//         meta.enable_equality(col_b);
//         meta.enable_equality(col_c);
//         meta.enable_equality(instance[0]);

//         // Defining a create_gate here applies it over every single column in the circuit
//         meta.create_gate("Fibonacci", |meta| {
//             let a = meta.query_advice(col_a, Rotation::cur());
//             let b = meta.query_advice(col_b, Rotation::cur());
//             let c = meta.query_advice(col_c, Rotation::cur());

//             // We will use the selector column to decide when to turn this gate on and off, since we probably don't want it on every row
//             let s = meta.query_selector(selector);

//             // a + b = c
//             vec![s * (a + b - c)]
//         });

//         FibonacciConfig {
//             advice: [col_a, col_b, col_c],
//             selector,
//             instance,
//         }
//     }

//     // These assign functions are to be called by the synthesizer, and will be used to assign values to the columns (the witness)
//     // The layouter will collect all the region definitions and compress it horizontally (i.e. squeeze up/down)
//     // but not vertically (i.e. will not squeeze left/right, at least right now)
//     fn assign_first_row(
//         &self,
//         mut layouter: impl Layouter<F>,
//         a: Option<F>,
//         b: Option<F>,
//     ) -> Result<(ACell<F>, ACell<F>, ACell<F>), Error> {
//         layouter.assign_region(
//             || "first row",
//             |mut region| {
//                 self.config.selector.enable(&mut region, 0)?;
//                 // let a_cell = region.assign_advice_from_instance(|| "1", self.config.instance[0], 0, self.config.advice[0], 0);
//                 let a_cell = region
//                     .assign_advice(
//                         || "a",
//                         self.config.advice[0],
//                         0,
//                         // || a.ok_or(Error::Synthesis),
//                         || a.unwrap(),
//                         // a.value().copied(),
//                     )
//                     .map(ACell)?;
//                 let b_cell = region
//                     .assign_advice(
//                         || "b",
//                         self.config.advice[1],
//                         0,
//                         || b.ok_or(Error::Synthesis),
//                     )
//                     .map(ACell)?;
//                 let c_val = a.and_then(|a| b.map(|b| a + b));

//                 let c_cell = region
//                     .assign_advice(
//                         || "c",
//                         self.config.advice[2],
//                         0,
//                         || c_val.ok_or(Error::Synthesis),
//                     )
//                     .map(ACell)?;
//                 Ok((a_cell, b_cell, c_cell))
//             },
//         )
//     }

//     // This will be repeatedly called. Note that each time it makes a new region, comprised of a, b, c, s that happen to all be in the same row
//     fn assign_row(
//         &self,
//         mut layouter: impl Layouter<F>,
//         prev_b: &ACell<F>,
//         prev_c: &ACell<F>,
//     ) -> Result<ACell<F>, Error> {
//         layouter.assign_region(
//             || "next row",
//             |mut region| {
//                 self.config.selector.enable(&mut region, 0)?;
//                 prev_b
//                     .0
//                     .copy_advice(|| "a", &mut region, self.config.advice[0], 0)?;
//                 prev_c
//                     .0
//                     .copy_advice(|| "b", &mut region, self.config.advice[1], 0)?;
//                 let c_val = prev_b
//                     .0
//                     .value()
//                     .and_then(|b| prev_c.0.value().map(|c| *b + *c));
//                 let c_cell = region
//                     .assign_advice(
//                         || "c",
//                         self.config.advice[2],
//                         0,
//                         || c_val.ok_or(Error::Synthesis),
//                     )
//                     .map(ACell);
//                 Ok(c_cell)
//             },
//         )?
//     }

//     pub fn expose_public(&self, mut layouter: impl Layouter<F>, cell: &ACell<F>, row: usize) {
//         layouter.constrain_instance(cell.0.cell(), self.config.instance[0], row);
//     }
// }

// // Note that the values in Circuit can be anything -- options, values, u64s, arbitrary objects, whatever.
// // As long as it's converted to the appropriate field elements in the assign functions called from synthesize, it's fine
// // Recall that circuits can call multiple chips in configure if they'd like!
// #[derive(Default)]
// struct FibonacciCircuit<F: FieldExt> {
//     pub a: Option<F>,
//     pub b: Option<F>,
// }

// // Our circuit will instantiate an instance based on the interface defined on the chip and floorplanner (layouter)
// // There isn't a clear reason this and the chip aren't the same thing, except for better abstractions for complex circuits
// impl<F: FieldExt> Circuit<F> for FibonacciCircuit<F> {
//     type Config = FibonacciConfig;
//     type FloorPlanner = SimpleFloorPlanner;

//     // Circuit without witnesses, called only during key generation
//     fn without_witnesses(&self) -> Self {
//         Self::default()
//     }

//     // Has the arrangement of columns. Called only during keygen, and will just call chip config most of the time
//     fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
//         let col_a: Column<Advice> = meta.advice_column();
//         let col_b: Column<Advice> = meta.advice_column();
//         let col_c: Column<Advice> = meta.advice_column();
//         let instance = meta.instance_column();
//         // One reason to configure with columns that are initialized in the Circuit instead of the Chip
//         // Is that you might want to share these columns between many chips -- it's easy to just pass the same column into both
//         FibonacciChip::configure(meta, [col_a, col_b, col_c], [instance])
//     }

//     // Take the output of configure and floorplanner type to make the actual circuit
//     // Called both at key generation time, and proving time with a specific witness
//     // Will call all of the copy constraints
//     fn synthesize(
//         &self,
//         config: Self::Config,
//         mut layouter: impl Layouter<F>,
//         // region: &mut Region<'_, F>,
//     ) -> Result<(), Error> {
//         let chip = FibonacciChip::construct(config);
//         let (mut prev_a, mut prev_b, mut prev_c) =
//             chip.assign_first_row(layouter.namespace(|| "first row"), self.a, self.b)?; // 2 private inputs

//         // Define the copy constraint from the instance column to our relevant advice cell
//         chip.expose_public(layouter.namespace(|| "private a"), &prev_a, 0);
//         chip.expose_public(layouter.namespace(|| "private b"), &prev_b, 1);
//         for _i in 3..10 {
//             let c_cell = chip.assign_row(layouter.namespace(|| "next row"), &prev_b, &prev_c)?;
//             prev_b = prev_c;
//             prev_c = c_cell;
//         }
//         // Define the copy constraint from the instance column to our relevant advice cell
//         chip.expose_public(layouter.namespace(|| "out"), &prev_c, 2); // Why is row 2 instead of 10?
//         Ok(())
//     }
// }

// fn main() {
//     let k = 4;
//     let a = Fp::from(1);
//     let b = Fp::from(1);
//     let out = Fp::from(55);
//     let circuit = FibonacciCircuit {
//         a: Some(a),
//         b: Some(b),
//     };
//     let mut public_inputs = vec![a, b, out];
//     // This prover is faster and 'fake', but is mostly a devtool for debugging
//     let prover = MockProver::run(k, &circuit, vec![public_inputs.clone()]).unwrap();
//     // This function will pretty-print on errors
//     prover.assert_satisfied();
//     public_inputs[2] += Fp::one();
//     let prover = MockProver::run(k, &circuit, vec![public_inputs.clone()]).unwrap();
//     // prover.assert_satisfied();
// }
