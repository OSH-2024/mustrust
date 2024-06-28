
//const int pdFALSE = 0;
//const int pdTRUE = 1;
#[macro_export]
macro_rules! pdFALSE { () => { 0 } }
#[macro_export]
macro_rules! pdTRUE { () => { 1 } }
//const int errQUEUE_EMPTY = 0;
//const int errQUEUE_FULL = 0;
#[macro_export]
macro_rules! errQUEUE_EMPTY { () => { 0 } }
#[macro_export]
macro_rules! errQUEUE_FULL { () => { 1 } }

/* FreeRTOS error definitions. */
//const int errCOULD_NOT_ALLOCATE_REQUIRED_MEMORY = -1;
//const int errQUEUE_BLOCKED = -4;
//const int errQUEUE_YIELD = -5;
#[macro_export]
macro_rules! errCOULD_NOT_ALLOCATE_REQUIRED_MEMORY { () => { -1 } }
#[macro_export]
macro_rules! errQUEUE_BLOCKED { () => { -4 } }
#[macro_export]
macro_rules! errQUEUE_YIELD { () => { -5 } }

/* Macros used for basic data corruption checks. */
//const int configUSE_LIST_DATA_INTEGRITY_CHECK_BYTES = 0;
#[macro_export]
macro_rules! configUSE_LIST_DATA_INTEGRITY_CHECK_BYTES { () => { 0 } }

//const int pdINTEGRITY_CHECK_VALUE = 0x5a5a5a5aUL;
#[macro_export]
macro_rules! pdINTEGRITY_CHECK_VALUE { () => { 0x5a5a5a5aUL } }

#[macro_export]
macro_rules! pdFREERTOS_ERRNO_NONE { () => { 0 } }	/* No errors */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_ENOENT { () => { 2 } }	/* No such file or directory */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EINTR { () => { 4 } }	/* Interrupted system call */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EIO { () => { 5 } }	/* I/O error */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_ENXIO { () => { 6 } }	/* No such device or address */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EBADF { () => { 9 } }	/* Bad file number */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EAGAIN { () => { 11 } }	/* No more processes */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EWOULDBLOCK { () => { 11 } }	/* Operation would block */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_ENOMEM { () => { 12 } }	/* Not enough memory */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EACCES { () => { 13 } }	/* Permission denied */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EFAULT { () => { 14 } }	/* Bad address */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EBUSY { () => { 16 } }	/* Mount device busy */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EEXIST { () => { 17 } }	/* File exists */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EXDEV { () => { 18 } }	/* Cross-device link */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_ENODEV { () => { 19 } }	/* No such device */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_ENOTDIR { () => { 20 } }	/* Not a directory */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EISDIR { () => { 21 } }	/* Is a directory */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EINVAL { () => { 22 } }	/* Invalid argument */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_ENOSPC { () => { 28 } }	/* No space left on device */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_ESPIPE { () => { 29 } }	/* Illegal seek */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EROFS { () => { 30 } }	/* Read only file system */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EUNATCH { () => { 42 } }	/* Protocol driver not attached */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EBADE { () => { 50 } }	/* Invalid exchange */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EFTYPE { () => { 79 } }	/* Inappropriate file type or format */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_ENMFILE { () => { 89 } }	/* No more files */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_ENOTEMPTY { () => { 90 } }	/* Directory not empty */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_ENAMETOOLONG { () => { 91 } }	/* File or path name too long */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EOPNOTSUPP { () => { 95 } }	/* Operation not supported on transport endpoint */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_ENOBUFS { () => { 105 } }	/* No buffer space available */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_ENOPROTOOPT { () => { 109 } }	/* Protocol not available */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EADDRINUSE { () => { 112 } }	/* Address already in use */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_ETIMEDOUT { () => { 116 } }	/* Connection timed out */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EINPROGRESS { () => { 119 } }	/* Connection already in progress */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EALREADY { () => { 120 } }	/* Socket already connected */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EADDRNOTAVAIL { () => { 125 } }	/* Address not available */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EISCONN { () => { 127 } }	/* Socket is already connected */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_ENOTCONN { () => { 128 } }	/* Socket is not connected */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_ENOMEDIUM { () => { 135 } }	/* No medium inserted */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_EILSEQ { () => { 138 } }	/* An invalid UTF-16 sequence was encountered. */
#[macro_export]
macro_rules! pdFREERTOS_ERRNO_ECANCELED { () => { 140 } }	/* Operation canceled. */

/* The following endian values are used by FreeRTOS+ components, not FreeRTOS
itself. */
#[macro_export]
macro_rules! pdFREERTOS_LITTLE_ENDIAN { () => { 0 } }
#[macro_export]
macro_rules! pdFREERTOS_BIG_ENDIAN { () => { 1 } }

/* Re-defining endian values for generic naming. */
#[macro_export]
macro_rules! pdLITTLE_ENDIAN { () => { pdFREERTOS_LITTLE_ENDIAN!() } }
#[macro_export]
macro_rules! pdBIG_ENDIAN { () => { pdFREERTOS_BIG_ENDIAN!() } }


#[macro_export]
macro_rules! portMAX_DELAY { () => { 0xffffffffffffffff as u64 } }

#[macro_export]
macro_rules! tskIDLE_PRIORITY { () => { 0 } }

#[macro_export]
macro_rules! portTICK_RATE_MS { () => { portTICK_PERIOD_MS!() } }
