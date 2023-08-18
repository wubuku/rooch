// <autogenerated>
//   This file was generated by dddappp code generator.
//   Any changes made to this file manually will be lost next time the file is regenerated.
// </autogenerated>

package org.test.roochblogdemo.domain.article;

import java.util.*;
import java.math.BigInteger;
import java.util.Date;
import org.test.roochblogdemo.domain.*;
import org.test.roochblogdemo.specialization.Event;

public interface CommentEvent extends Event {

    interface SqlCommentEvent extends CommentEvent {
        CommentEventId getCommentEventId();

        boolean getEventReadOnly();

        void setEventReadOnly(boolean readOnly);
    }

    BigInteger getCommentSeqId();

    //void setCommentSeqId(BigInteger commentSeqId);

    String getCreatedBy();

    void setCreatedBy(String createdBy);

    Date getCreatedAt();

    void setCreatedAt(Date createdAt);

    String getCommandId();

    void setCommandId(String commandId);


}

