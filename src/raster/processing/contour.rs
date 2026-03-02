use std::ptr;

use gdal_sys::CPLErr;

use crate::cpl::CslStringList;
use crate::errors::Result;
use crate::raster::RasterBand;
use crate::utils::_last_cpl_err;
use crate::vector::LayerAccess;

/// Generate contour lines or polygons from a raster band into an OGR layer.
///
/// Wraps [`GDALContourGenerateEx`](https://gdal.org/api/gdal_alg.html#_CPPv422GDALContourGenerateEx16GDALRasterBandHP12OGRLayerH12CSLConstList16GDALProgressFuncPv).
///
/// The caller is responsible for creating the output layer with appropriate
/// geometry type and fields. The `options` parameter controls contour
/// generation behavior via key=value pairs:
///
/// - `LEVEL_INTERVAL` — contour interval
/// - `LEVEL_BASE` — base contour level
/// - `FIXED_LEVELS` — comma-separated list of explicit contour levels
/// - `NODATA` — override nodata value
/// - `ID_FIELD` — output field name for feature ID
/// - `ELEV_FIELD` — output field name for elevation (linestring mode)
/// - `ELEV_FIELD_MIN` / `ELEV_FIELD_MAX` — output field names for min/max
///   elevation (polygon mode)
/// - `POLYGONIZE=YES` — produce polygons instead of linestrings
///
/// # Example
///
/// ```rust, no_run
/// # fn main() -> gdal::errors::Result<()> {
/// use gdal::Dataset;
/// use gdal::DriverManager;
/// use gdal::cpl::CslStringList;
/// use gdal::vector::{LayerAccess, LayerOptions};
/// use gdal::raster::processing::contour::contour_generate;
///
/// let ds = Dataset::open("input.tif")?;
/// let band = ds.rasterband(1)?;
///
/// let mem_driver = DriverManager::get_driver_by_name("Memory")?;
/// let mut mem_ds = mem_driver.create_vector_only("")?;
/// let mut layer = mem_ds.create_layer(LayerOptions {
///     name: "contours",
///     ty: gdal_sys::OGRwkbGeometryType::wkbMultiPolygon,
///     ..Default::default()
/// })?;
///
/// let mut opts = CslStringList::new();
/// opts.add_string("FIXED_LEVELS=0.005,0.05,0.5")?;
/// opts.add_string("POLYGONIZE=YES")?;
/// opts.add_string("ELEV_FIELD_MIN=ELEV_MIN")?;
/// opts.add_string("ELEV_FIELD_MAX=ELEV_MAX")?;
/// opts.add_string("ID_FIELD=ID")?;
///
/// contour_generate(&band, &layer, &opts)?;
///
/// for feature in layer.features() {
///     let geom = feature.geometry().unwrap();
///     let wkb = geom.wkb()?;
///     // ... use WKB geometry
/// }
/// # Ok(())
/// # }
/// ```
pub fn contour_generate<L: LayerAccess>(
    band: &RasterBand,
    layer: &L,
    options: &CslStringList,
) -> Result<()> {
    let rv = unsafe {
        gdal_sys::GDALContourGenerateEx(
            band.c_rasterband(),
            layer.c_layer() as *mut std::ffi::c_void,
            options.as_ptr(),
            None,
            ptr::null_mut(),
        )
    };
    if rv != CPLErr::CE_None {
        return Err(_last_cpl_err(rv));
    }
    Ok(())
}
