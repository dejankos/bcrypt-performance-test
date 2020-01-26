## Simple testing of bcrypt-4 
Jmeter test script included.

Tested with latest rust stable 1.40.0 (73528e339 2019-12-16), 5.0.0-32-generic #34~18.04.2-Ubuntu on Dell XPS 15 9570

Couldn't find a crate that performs hashing/verify under ~10ms.
Tested with crates: pwhash, rust-bcrypt, rust-djangohashers and bcrypt-small-rs.

Final result with rust-bcrypt.
### Result
```
summary +   2477 in 00:00:05 =  496.5/s Avg:   359 Min:    16 Max:   832 Err:     0 (0.00%) Active: 210 Started: 210 Finished: 0
summary +  14505 in 00:00:30 =  483.4/s Avg:   433 Min:    70 Max:  1086 Err:     0 (0.00%) Active: 210 Started: 210 Finished: 0
summary =  16982 in 00:00:35 =  485.3/s Avg:   422 Min:    16 Max:  1086 Err:     0 (0.00%)
summary +   4018 in 00:00:09 =  443.9/s Avg:   427 Min:    14 Max:  1364 Err:     0 (0.00%) Active: 0 Started: 210 Finished: 210
summary =  21000 in 00:00:44 =  476.8/s Avg:   423 Min:    14 Max:  1364 Err:     0 (0.00%)
```