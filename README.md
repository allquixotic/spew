# spew - A data destruction tool for MacOS

Spew is a simple data destruction tool designed for MacOS (although it can easily be adapted to Linux with an ioctl change).

The main use cases of this tool are:

 - Destroy random sectors of data on an HDD to discourage data recovery attempts without spending the time to completely wipe the data
 - Observe the change in disk throughput with different seek frequencies / block sizes (e.g. when looking at HDD performance)

 