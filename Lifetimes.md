fn iter<'a>(&'a self) -> Iter<'a, T> {
	// Impl
}

Here, if we call this function then reference to the self should be alive till the returned Iter instance stays alive.



