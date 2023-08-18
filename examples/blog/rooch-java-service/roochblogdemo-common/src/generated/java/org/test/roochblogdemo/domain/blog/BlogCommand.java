// <autogenerated>
//   This file was generated by dddappp code generator.
//   Any changes made to this file manually will be lost next time the file is regenerated.
// </autogenerated>

package org.test.roochblogdemo.domain.blog;

import java.util.*;
import java.util.Date;
import java.math.BigInteger;
import org.test.roochblogdemo.domain.*;
import org.test.roochblogdemo.domain.Command;
import org.test.roochblogdemo.specialization.DomainError;

public interface BlogCommand extends Command {

    String getAccountAddress();

    void setAccountAddress(String accountAddress);

    Long getOffChainVersion();

    void setOffChainVersion(Long offChainVersion);

    static void throwOnInvalidStateTransition(BlogState state, Command c) {
        if (state.getOffChainVersion() == null) {
            if (isCommandCreate((BlogCommand)c)) {
                return;
            }
            throw DomainError.named("premature", "Can't do anything to unexistent aggregate");
        }
        if (state.getDeleted() != null && state.getDeleted()) {
            throw DomainError.named("zombie", "Can't do anything to deleted aggregate.");
        }
        if (isCommandCreate((BlogCommand)c))
            throw DomainError.named("rebirth", "Can't create aggregate that already exists");
    }

    static boolean isCommandCreate(BlogCommand c) {
        if (c.getOffChainVersion().equals(BlogState.VERSION_NULL))
            return true;
        return false;
    }

}

