//
// Tests for single-use-variable elimination
//

// Check forward movement of values
#[test="simple_fwd_exp"]
fn simple_fwd(a: i32) -> (i32,)
{
	let v: i32;
	bb0: {
		ASSIGN v = a;
		ASSIGN retval = (v,);
	} RETURN;
}
fn simple_fwd_exp(a: i32) -> (i32,)
{
	bb0: {
		ASSIGN retval = (a,);
	} RETURN;
}

// Reverse (upwards) movement
#[test="simple_rev_exp"]
fn simple_rev(a: i32) -> (i32,)
{
	let v: (i32,);
	bb0: {
		ASSIGN v = (a,);
		ASSIGN retval = v;
	} RETURN;
}
fn simple_rev_exp(a: i32) -> (i32,)
{
	bb0: {
		ASSIGN retval = (a,);
	} RETURN;
}

// Check that if there's a mutable borrow of the source, that the source isn't propagated forwards
// - NOTE: This relies on the optimiser not being smart enough to move the `retval` construction up
#[test="nomut"]
fn nomut(a: i32) -> (i32,)
{
	let v: i32;
	let ba: &mut i32;
	bb0: {
		ASSIGN v = a;
		ASSIGN ba = &mut a;
		//ASSIGN ba* = +0;	// This could be here, which is why the optimisation is unsound
		ASSIGN retval = (v,);
		DROP ba;
	} RETURN;
}

/*	// TODO: This doesn't pass yet (can't move the assignment up it seems)
// NOTE: Test based on sample from `<::"alloc"::rc::Rc<[u8],>>::allocate_for_ptr`
// Reverse (upwards) movement
#[test="borrowed_rev_exp"]
fn borrowed_rev(a: &mut [u8], a2: &mut u8)
{
	let var11: *mut [u8];
	let var21: *mut u8;
	let var30: *mut [u8];
	let var22: &mut *mut [u8];
	bb0: {
		ASSIGN var11 = CAST a as *mut [u8];
		ASSIGN var21 = CAST a2 as *mut u8;
		ASSIGN var30 = var11;
		ASSIGN var22 = &mut var30;
	} CALL retval = "black_box"(var22, var21) => bb1 else bb2;
	bb1: {
	} RETURN;
	bb2: {
	} DIVERGE;
}
fn borrowed_rev_exp(a: &mut [u8], a2: &mut u8)
{
	let var21: *mut u8;
	let var30: *mut [u8];
	let var22: &mut *mut [u8];
	bb0: {
		ASSIGN var30 = CAST a as *mut [u8];
		ASSIGN var21 = CAST a2 as *mut u8;
		ASSIGN var22 = &mut var30;
	} CALL retval = "black_box"(var22, var21) => bb1 else bb2;
	bb1: {
	} RETURN;
	bb2: {
	} DIVERGE;
}
//*/