// <autogenerated>
//   This file was generated by dddappp code generator.
//   Any changes made to this file manually will be lost next time the file is regenerated.
// </autogenerated>

package org.test.roochblogdemo.domain.blog;

import java.util.*;
import java.util.Date;
import java.math.BigInteger;
import org.test.roochblogdemo.domain.*;
import org.test.roochblogdemo.specialization.Event;

public interface BlogEvent extends Event, RoochEvent, HasStatus {

    interface SqlBlogEvent extends BlogEvent {
        BlogEventId getBlogEventId();

        boolean getEventReadOnly();

        void setEventReadOnly(boolean readOnly);
    }

    String getAccountAddress();

    //void setAccountAddress(String accountAddress);

    BigInteger getVersion();
    
    //void setVersion(BigInteger version);

    String getCreatedBy();

    void setCreatedBy(String createdBy);

    Date getCreatedAt();

    void setCreatedAt(Date createdAt);

    String getCommandId();

    void setCommandId(String commandId);


}

