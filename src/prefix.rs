/// Macro to implement the [SI][si] decimal and [IEC][iec] binary prefixes for
/// [multiples of units][mult] and [submultiples of units][submult].
///
/// Implemented using `macro_rules!` instead of `const` so that type inference at call sites can
/// generate the appropriate float type. Using explicit constants would require duplicate
/// definitions for `f32` and `f64` or casting from `f64` in `f32` contexts.
///
/// [si]: http://jcgm.bipm.org/vim/en/1.16.html
/// [iec]: http://www.iec.ch/
/// [mult]: http://jcgm.bipm.org/vim/en/1.17.html
/// [submult]: http://jcgm.bipm.org/vim/en/1.18.html
#[macro_export]
macro_rules! prefix {
    (yobi) => { (1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0) };
    (yotta) => { 1.0_E24 };
    (zebi) => { (1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0) };
    (zetta) => { 1.0_E21 };
    (exbi) => { (1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0) };
    (exa) => { 1.0_E18 };
    (pebi) => { (1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0) };
    (peta) => { 1.0_E15 };
    (tebi) => { (1024.0 * 1024.0 * 1024.0 * 1024.0) };
    (tera) => { 1.0_E12 };
    (gibi) => { (1024.0 * 1024.0 * 1024.0) };
    (giga) => { 1.0_E9 };
    (mebi) => { (1024.0 * 1024.0) };
    (mega) => { 1.0_E6 };
    (kibi) => { 1024.0 };
    (kilo) => { 1.0_E3 };
    (hecto) => { 1.0_E2 };
    (deca) => { 1.0_E1 };
    (none) => { 1.0_E0 };
    (deci) => { 1.0_E-1 };
    (centi) => { 1.0_E-2 };
    (milli) => { 1.0_E-3 };
    (micro) => { 1.0_E-6 };
    (nano) => { 1.0_E-9 };
    (pico) => { 1.0_E-12 };
    (femto) => { 1.0_E-15 };
    (atto) => { 1.0_E-18 };
    (zepto) => { 1.0_E-21 };
    (yocto) => { 1.0_E-24 };
}
