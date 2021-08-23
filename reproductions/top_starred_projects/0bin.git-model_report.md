# Model report for file:///tmp/top-repos-quality-repos-v3vkr_ue/0bin.git HEAD 3e0c4da19302bee38f6cb009362e2f7071b88705

### Dump

```json
{'created_at': '2021-08-23 06:48:12',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.9 kB',
 'tags': [],
 'uuid': 'c44c9f49-af81-472e-a103-46f206662d4c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-v3vkr_ue/0bin.git 3e0c4da19302bee38f6cb009362e2f7071b88705

# javascript
39 rules, avg.len. 8.2
## train
PPCR: 0.960542
### report
macro
{'f1-score': 0.8728929328231516,
 'precision': 0.9228140578895183,
 'recall': 0.8396974548000333,
 'support': 73444}
micro
{'f1-score': 0.9576820434616851,
 'precision': 0.9576820434616851,
 'recall': 0.9576820434616851,
 'support': 73444}
weighted
{'f1-score': 0.9564199914694779,
 'precision': 0.9571343690780844,
 'recall': 0.9576820434616851,
 'support': 73444}
### report_full
macro
{'f1-score': 0.8159833892245901,
 'precision': 0.9228140578895183,
 'recall': 0.755971064980502,
 'support': 76461}
micro
{'f1-score': 0.9384076581835162,
 'precision': 0.9576820434616851,
 'recall': 0.9198938020690287,
 'support': 76461}
weighted
{'f1-score': 0.9333795629074314,
 'precision': 0.9555749595285606,
 'recall': 0.9198938020690287,
 'support': 76461}
## test
PPCR: 0.962524
### report
macro
{'f1-score': 0.7463564726545688,
 'precision': 0.7898960157241834,
 'recall': 0.7160309767105446,
 'support': 6421}
micro
{'f1-score': 0.9120074754711104,
 'precision': 0.9120074754711104,
 'recall': 0.9120074754711104,
 'support': 6421}
weighted
{'f1-score': 0.9133116011412032,
 'precision': 0.9193364967766354,
 'recall': 0.9120074754711104,
 'support': 6421}
### report_full
macro
{'f1-score': 0.7048213546567244,
 'precision': 0.7898960157241834,
 'recall': 0.6541217241961286,
 'support': 6671}
micro
{'f1-score': 0.8945921173235563,
 'precision': 0.9120074754711104,
 'recall': 0.8778294108829261,
 'support': 6671}
weighted
{'f1-score': 0.8922288479700984,
 'precision': 0.9176331747906518,
 'recall': 0.8778294108829261,
 'support': 6671}
```

## javascript
### Summary
26 rules, avg.len. 8.1

| | |
|-|-|
|Min support|92|
|Max support|9956|
|Min confidence|0.9221311211585999|
|Max confidence|0.9995693564414978|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.987. Support: 9220.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {+}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.946. Support: 139.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 405.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 366.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 167.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = !<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 120.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, (, {}<br>	∧ -4.length ≤ 30<br>	∧ +1.reserved not in {), ]}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 7221.` |
| 8 | `  -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1161.` |
| 9 | `  -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION, NAME} and not in {STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 374.` |
| 10 | `  -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION} and not in {NAME, STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 92.` |
| 11 | `  -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION} and not in {NAME, STRING}<br>	∧ -2.length ≥ 3<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.996. Support: 122.` |
| 12 | `  -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION} and not in {NAME, STRING}<br>	∧ -2.length ≥ 3<br>	∧ +1.reserved not in {), =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 1087.` |
| 13 | `  -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION} and not in {NAME, STRING}<br>	∧ -2.length ≤ 2<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 8074.` |
| 14 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.940. Support: 108.` |
| 15 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 4198.` |
| 16 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1905.` |
| 17 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1253.` |
| 18 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, SCOPE}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 197.` |
| 19 | `  •••start_col ≤ 6<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, SCOPE}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.980. Support: 125.` |
| 20 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type = CommentBlock<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 210.` |
| 21 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {CommentBlock}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.924. Support: 1068.` |
| 22 | `  -1.diff_col ≤ 8<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 247.` |
| 23 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = !<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 201.` |
| 24 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 113.` |
| 25 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, (, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {KEY}<br>	∧ -3.label in {<newline>}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 351.` |
| 26 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, (, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 9956.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.076923076923077, "max_conf": 0.9995693564414978, "max_support": 9956, "min_conf": 0.9221311211585999, "min_support": 92, "num_rules": 26}}
```
</details>
