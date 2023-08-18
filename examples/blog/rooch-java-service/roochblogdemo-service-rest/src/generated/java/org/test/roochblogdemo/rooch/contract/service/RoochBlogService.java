// <autogenerated>
//   This file was generated by dddappp code generator.
//   Any changes made to this file manually will be lost next time the file is regenerated.
// </autogenerated>

package org.test.roochblogdemo.rooch.contract.service;

import com.github.wubuku.rooch.utils.RoochJsonRpcClient;
import org.test.roochblogdemo.domain.*;
import org.test.roochblogdemo.domain.blog.*;
import org.test.roochblogdemo.rooch.contract.repository.*;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.stream.*;
import java.util.*;
import java.math.*;

@Service
public class RoochBlogService {

    @Autowired
    private BlogStateRepository blogStateRepository;


    private RoochBlogStateRetriever roochBlogStateRetriever;

    @Autowired
    public RoochBlogService(RoochJsonRpcClient roochJsonRpcClient) {
        this.roochBlogStateRetriever = new RoochBlogStateRetriever(roochJsonRpcClient,
                accountAddress -> {
                    BlogState.MutableBlogState s = new AbstractBlogState.SimpleBlogState();
                    s.setAccountAddress(accountAddress);
                    return s;
                }
        );
    }

    @Transactional
    public void updateBlogState(String accountAddress) {
        BlogState blogState = roochBlogStateRetriever.retrieveBlogState(accountAddress);
        if (blogState == null) {
            return;
        }
        blogStateRepository.merge(blogState);
    }

    @Transactional
    public void deleteBlog(String accountAddress) {
        BlogState.MutableBlogState s = (BlogState.MutableBlogState) blogStateRepository.get(accountAddress, true);
        if (s != null) {
            s.setDeleted(true);
            blogStateRepository.merge(s);
        }
    }

}

