# Model report for file:///tmp/top-repos-quality-repos-wyiob0e8/zecwallet.git HEAD 3f59978ebbf0a67a7ee70d103f5b08c34b191ae7

### Dump

```json
{'created_at': '2021-08-23 04:49:40',
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
 'size': '17.4 kB',
 'tags': [],
 'uuid': '0dedc3d0-cc33-45b1-aa39-3142481187f6',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-wyiob0e8/zecwallet.git 3f59978ebbf0a67a7ee70d103f5b08c34b191ae7

# javascript
26 rules, avg.len. 8.0
## train
PPCR: 0.830103
### report
macro
{'f1-score': 0.7740002971622459,
 'precision': 0.8136062566898076,
 'recall': 0.7456924244851825,
 'support': 21029}
micro
{'f1-score': 0.9380379475961768,
 'precision': 0.9380379475961768,
 'recall': 0.9380379475961768,
 'support': 21029}
weighted
{'f1-score': 0.9346900592335736,
 'precision': 0.9340019467309166,
 'recall': 0.9380379475961768,
 'support': 21029}
### report_full
macro
{'f1-score': 0.6273250824028763,
 'precision': 0.8136062566898076,
 'recall': 0.546493015380493,
 'support': 25333}
micro
{'f1-score': 0.8509555239204522,
 'precision': 0.9380379475961768,
 'recall': 0.778668140370268,
 'support': 25333}
weighted
{'f1-score': 0.8336629210534592,
 'precision': 0.932628864751814,
 'recall': 0.778668140370268,
 'support': 25333}
## test
PPCR: 0.851277
### report
macro
{'f1-score': 0.7375344799264285,
 'precision': 0.8104054808927663,
 'recall': 0.6932571610759493,
 'support': 4934}
micro
{'f1-score': 0.9256181597081475,
 'precision': 0.9256181597081475,
 'recall': 0.9256181597081475,
 'support': 4934}
weighted
{'f1-score': 0.9203514103748948,
 'precision': 0.9212666776112961,
 'recall': 0.9256181597081475,
 'support': 4934}
### report_full
macro
{'f1-score': 0.6195592545217151,
 'precision': 0.8104054808927663,
 'recall': 0.5362136817189719,
 'support': 5796}
micro
{'f1-score': 0.8512581547064306,
 'precision': 0.9256181597081475,
 'recall': 0.7879572118702554,
 'support': 5796}
weighted
{'f1-score': 0.8341572521412646,
 'precision': 0.9196515939988767,
 'recall': 0.7879572118702554,
 'support': 5796}
```

## javascript
### Summary
17 rules, avg.len. 8.7

| | |
|-|-|
|Min support|92|
|Max support|8553|
|Min confidence|0.9206587076187134|
|Max confidence|0.9990000128746033|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.960. Support: 796.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.999. Support: 500.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 307.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.985. Support: 103.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION}<br>	∧ ^2.roles in {TYPE}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 92.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -3.length ≥ 3<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {DECLARATION}<br>	∧ ^2.roles not in {TYPE}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 675.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 257.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 302.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≤ 6<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.950. Support: 372.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.length ≥ 3<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 334.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.length ≤ 2<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {BLOCK, DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 205.` |
| 12 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.951. Support: 194.` |
| 13 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.reserved = ;<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 250.` |
| 14 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {;}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 300.` |
| 15 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved not in {;}<br>	∧ -4.label not in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE} and not in {CONDITION, DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 310.` |
| 16 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved not in {;}<br>	∧ -4.label not in {<-space>}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {IDENTIFIER} and not in {LITERAL}<br>	∧ +2.reserved = ><br>	∧ ^1.roles not in {CONDITION, DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 195.` |
| 17 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved not in {;}<br>	∧ -4.label not in {<-space>}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=, >}<br>	∧ ^1.roles not in {CONDITION, DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 8553.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.705882352941176, "max_conf": 0.9990000128746033, "max_support": 8553, "min_conf": 0.9206587076187134, "min_support": 92, "num_rules": 17}}
```
</details>
