hierarchical timewheel for async timers, scales really well but not precise, it was made for a thread/core async runtime.
- timers can be cancelled
- well otpimized
- really few dyn allocations during runtime
- Duration < 24h
- meant to operate on worker thread so the thread can be busy and miss timings
