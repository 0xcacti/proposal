## Proposal 

Proposal is a tool to check and see, for a given epoch, if a given validator will propose one or more blocks.  

get - returns a list of proposing validator IDs for the current epoch
check- accepts a list of validator IDs and an epoch number (current or next) and returns a mapping of validator IDs to bools representing if the validator will propose or not.
