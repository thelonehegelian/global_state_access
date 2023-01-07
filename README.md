# global_state_access
Creates a global data for access to avoid race conditions using Mutex and Mutex.lock

 Explanantion for why Mango is added last though it appears first in the code:
 * The order in which the items are added to the vector is not guaranteed because there are multiple threads modifying the vector concurrently. In this case, the main thread and the thread spawned on line 29 both try to acquire a lock on the Mutex and modify the vector. The order in which they acquire the lock and modify the vector is not deterministic and can vary between runs. This is what leads to the seemingly random order in which the items are added to the vector.
