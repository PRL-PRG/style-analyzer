# Model report for file:///tmp/top-repos-quality-repos-nbgsvwuk/birdseye.git HEAD 0f0e0b40e16584d232587e0cffec41660a158aa0

### Dump

```json
{'created_at': '2021-08-29 23:36:28',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.9 kB',
 'tags': [],
 'uuid': '4de069f3-52da-424f-ae34-d63232217f33',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-nbgsvwuk/birdseye.git 0f0e0b40e16584d232587e0cffec41660a158aa0

# javascript
90 rules, avg.len. 5.8
## train
PPCR: 0.982590
### report
macro
{'f1-score': 0.6795261100604629,
 'precision': 0.6796374831882697,
 'recall': 0.6801912009657525,
 'support': 9538}
micro
{'f1-score': 0.9303837282449152,
 'precision': 0.9303837282449151,
 'recall': 0.9303837282449151,
 'support': 9538}
weighted
{'f1-score': 0.9158106252069755,
 'precision': 0.9025567782774968,
 'recall': 0.9303837282449151,
 'support': 9538}
### report_full
macro
{'f1-score': 0.6772531801297399,
 'precision': 0.6796374831882697,
 'recall': 0.6755322033188982,
 'support': 9707}
micro
{'f1-score': 0.9222135619641466,
 'precision': 0.9303837282449151,
 'recall': 0.9141856392294221,
 'support': 9707}
weighted
{'f1-score': 0.9033143342259456,
 'precision': 0.8933245745077438,
 'recall': 0.9141856392294221,
 'support': 9707}
## test
PPCR: 0.995274
### report
macro
{'f1-score': 0.3534278376648782,
 'precision': 0.34411103464612347,
 'recall': 0.36647507134806706,
 'support': 2738}
micro
{'f1-score': 0.8400292184075968,
 'precision': 0.8400292184075968,
 'recall': 0.8400292184075968,
 'support': 2738}
weighted
{'f1-score': 0.7933024919248834,
 'precision': 0.7534887454726371,
 'recall': 0.8400292184075968,
 'support': 2738}
### report_full
macro
{'f1-score': 0.3522992186905061,
 'precision': 0.34411103464612347,
 'recall': 0.363959870506745,
 'support': 2751}
micro
{'f1-score': 0.8380397157952268,
 'precision': 0.8400292184075968,
 'recall': 0.8360596146855689,
 'support': 2751}
weighted
{'f1-score': 0.7915697804526447,
 'precision': 0.7533703889250427,
 'recall': 0.8360596146855689,
 'support': 2751}
```

## javascript
### Summary
64 rules, avg.len. 4.8

| | |
|-|-|
|Min support|150|
|Max support|1950|
|Min confidence|0.9204545617103577|
|Max confidence|0.9987562298774719|

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
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
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
| 1 | `  -2.diff_col ≥ 9<br>	∧ -4.diff_offset ≥ 12<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 154.` |
| 2 | `  -2.diff_col ≥ 9<br>	∧ -4.diff_offset ≤ 11<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 151.` |
| 3 | `  -1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.997. Support: 159.` |
| 4 | `  -1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 1915.` |
| 5 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.976. Support: 226.` |
| 6 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.998. Support: 211.` |
| 7 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.969. Support: 244.` |
| 8 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {BLOCK} and not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.954. Support: 207.` |
| 9 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 257.` |
| 10 | `  -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 494.` |
| 11 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 472.` |
| 12 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.937. Support: 261.` |
| 13 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.980. Support: 179.` |
| 14 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 155.` |
| 15 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 816.` |
| 16 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 205.` |
| 17 | `  -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 175.` |
| 18 | `  -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 1869.` |
| 19 | `  -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 9<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.965. Support: 269.` |
| 20 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 8<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.991. Support: 170.` |
| 21 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 8<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {BLOCK} and not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.923. Support: 200.` |
| 22 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 8<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.988. Support: 215.` |
| 23 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 951.` |
| 24 | `  +2.roles in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 596.` |
| 25 | `  +1.reserved = ;<br>	∧ +2.roles not in {ARGUMENT}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 611.` |
| 26 | `  -1.reserved = (<br>	∧ +1.reserved not in {;}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 555.` |
| 27 | `  -1.reserved not in {(}<br>	∧ +1.reserved = )<br>⇒ y = ∅<br>Confidence: 0.970. Support: 517.` |
| 28 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {), ;}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.972. Support: 234.` |
| 29 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.998. Support: 214.` |
| 30 | `  -1.reserved = {<br>	∧ +1.reserved not in {), ;}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.993. Support: 210.` |
| 31 | `  -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 402.` |
| 32 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ +1.reserved not in {), ;, =}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles in {BINARY} and not in {ARGUMENT}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 192.` |
| 33 | `  -1.roles in {EXPRESSION}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 166.` |
| 34 | `  -1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 1950.` |
| 35 | `  -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 504.` |
| 36 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 480.` |
| 37 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 167.` |
| 38 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 193.` |
| 39 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.993. Support: 208.` |
| 40 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 343.` |
| 41 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 258.` |
| 42 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.977. Support: 150.` |
| 43 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 196.` |
| 44 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {BLOCK} and not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.936. Support: 226.` |
| 45 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.980. Support: 223.` |
| 46 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.969. Support: 210.` |
| 47 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 371.` |
| 48 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.994. Support: 231.` |
| 49 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {SCOPE} and not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.939. Support: 238.` |
| 50 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.981. Support: 235.` |
| 51 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {COMMENT, EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 228.` |
| 52 | `  -1.diff_offset ≥ 9<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.967. Support: 256.` |
| 53 | `  -1.diff_offset ≤ 8<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.986. Support: 178.` |
| 54 | `  -1.diff_offset ≤ 8<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.920. Support: 220.` |
| 55 | `  -1.diff_offset ≤ 8<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.991. Support: 176.` |
| 56 | `  -1.diff_offset ≤ 8<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 214.` |
| 57 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 1050.` |
| 58 | `  +2.roles in {ARGUMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 613.` |
| 59 | `  -1.reserved = (<br>	∧ +2.roles not in {ARGUMENT}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 571.` |
| 60 | `  -1.reserved not in {(}<br>	∧ +1.reserved = ;<br>⇒ y = ∅<br>Confidence: 0.979. Support: 554.` |
| 61 | `  -1.reserved not in {(}<br>	∧ +1.internal_type = CommentLine<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 240.` |
| 62 | `  -1.reserved not in {(}<br>	∧ -1.roles in {COMMENT}<br>	∧ +1.internal_type not in {CommentLine}<br>	∧ +1.reserved not in {), ;}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles in {FILE} and not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.976. Support: 232.` |
| 63 | `  -1.reserved = {<br>	∧ +1.reserved not in {), ;}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.988. Support: 204.` |
| 64 | `  -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 378.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.75, "max_conf": 0.9987562298774719, "max_support": 1950, "min_conf": 0.9204545617103577, "min_support": 150, "num_rules": 64}}
```
</details>
