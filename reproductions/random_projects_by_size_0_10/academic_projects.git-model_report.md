# Model report for file:///tmp/top-repos-quality-repos-io60klpb/academic_projects.git HEAD 16859c4d586c7d1200631e4aadee46a7bb518ecd

### Dump

```json
{'created_at': '2021-08-22 17:48:37',
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
 'size': '20.3 kB',
 'tags': [],
 'uuid': '762f72ab-4e84-49e9-bc57-457458ea280d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-io60klpb/academic_projects.git 16859c4d586c7d1200631e4aadee46a7bb518ecd

# javascript
31 rules, avg.len. 11.3
## train
PPCR: 0.918664
### report
macro
{'f1-score': 0.47279519968187494,
 'precision': 0.4804696443247109,
 'recall': 0.46631295430070946,
 'support': 83863}
micro
{'f1-score': 0.9671130295839643,
 'precision': 0.9671130295839643,
 'recall': 0.9671130295839643,
 'support': 83863}
weighted
{'f1-score': 0.9643633140900303,
 'precision': 0.9628263647152498,
 'recall': 0.9671130295839643,
 'support': 83863}
### report_full
macro
{'f1-score': 0.45109286884166866,
 'precision': 0.4804696443247109,
 'recall': 0.4289147330826469,
 'support': 91288}
micro
{'f1-score': 0.9261151806155832,
 'precision': 0.9671130295839643,
 'recall': 0.888451932345982,
 'support': 91288}
weighted
{'f1-score': 0.9087085993921202,
 'precision': 0.9338887397080142,
 'recall': 0.888451932345982,
 'support': 91288}
## test
PPCR: 0.747381
### report
macro
{'f1-score': 0.29699329950518777,
 'precision': 0.32390311300525765,
 'recall': 0.28194820881572674,
 'support': 3068}
micro
{'f1-score': 0.9452411994784876,
 'precision': 0.9452411994784876,
 'recall': 0.9452411994784876,
 'support': 3068}
weighted
{'f1-score': 0.9443974882292455,
 'precision': 0.9499836878766855,
 'recall': 0.9452411994784876,
 'support': 3068}
### report_full
macro
{'f1-score': 0.235886223316089,
 'precision': 0.32390311300525765,
 'recall': 0.2058266903646678,
 'support': 4105}
micro
{'f1-score': 0.8085877596542591,
 'precision': 0.9452411994784876,
 'recall': 0.7064555420219245,
 'support': 4105}
weighted
{'f1-score': 0.7725392074463622,
 'precision': 0.8935117963599953,
 'recall': 0.7064555420219245,
 'support': 4105}
```

## javascript
### Summary
24 rules, avg.len. 9.7

| | |
|-|-|
|Min support|124|
|Max support|15436|
|Min confidence|0.9333173036575317|
|Max confidence|0.9998599290847778|

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
| 1 | `  -1.reserved not in {[}<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 9078.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 1.000. Support: 3570.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.999. Support: 3016.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +2.roles in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 3633.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = ;<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 3215.` |
| 6 | `  •••start_line ≥ 253<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -3.diff_offset ≤ 15<br>	∧ +1.reserved = }<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.933. Support: 2077.` |
| 7 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.994. Support: 1867.` |
| 8 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -5.roles in {VALUE}<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.999. Support: 388.` |
| 9 | `  •••start_line ≥ 253<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -5.roles not in {VALUE}<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.935. Support: 1878.` |
| 10 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 2052.` |
| 11 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2040.` |
| 12 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {,, ;, }}<br>	∧ +1.roles in {IDENTIFIER} and not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 124.` |
| 13 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {MAP}<br>	∧ +1.reserved not in {,, ;, }}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 599.` |
| 14 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {MAP}<br>	∧ -2.reserved = }<br>	∧ +1.reserved not in {,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.988. Support: 382.` |
| 15 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = !<br>	∧ -1.roles not in {MAP}<br>	∧ -2.reserved not in {}}<br>	∧ +1.reserved not in {}}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 397.` |
| 16 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, ;, {}<br>	∧ -1.roles in {IDENTIFIER} and not in {MAP}<br>	∧ +1.reserved = (<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 906.` |
| 17 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, ;, {}<br>	∧ -1.roles not in {IDENTIFIER, MAP}<br>	∧ +1.reserved = (<br>	∧ +2.reserved = )<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 179.` |
| 18 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, {}<br>	∧ -1.roles not in {IDENTIFIER, MAP}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.reserved not in {)}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 1624.` |
| 19 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.997. Support: 162.` |
| 20 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {MAP}<br>	∧ +1.reserved = )<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 431.` |
| 21 | `  •••start_line ≥ 250<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {MAP}<br>	∧ +1.reserved not in {(, ), ,}<br>	∧ +1.roles not in {ASSIGNMENT, COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 3830.` |
| 22 | `  -1.diff_col ≤ 29<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {MAP}<br>	∧ -3.length ≤ 4<br>	∧ +1.reserved = ]<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 649.` |
| 23 | `  •••start_line ≥ 225<br>	∧ -1.diff_col ≤ 29<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {MAP}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 2865.` |
| 24 | `  -1.diff_col ≤ 29<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {MAP}<br>	∧ -2.reserved not in {}}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 15436.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.666666666666666, "max_conf": 0.9998599290847778, "max_support": 15436, "min_conf": 0.9333173036575317, "min_support": 124, "num_rules": 24}}
```
</details>
