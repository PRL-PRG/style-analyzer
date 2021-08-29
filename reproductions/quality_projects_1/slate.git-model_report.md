# Model report for file:///tmp/top-repos-quality-repos-16uny_w2/slate.git HEAD e3d77f11e042cbf6281455ed324a0e78986b37bb

### Dump

```json
{'created_at': '2021-08-29 01:45:02',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.19.0-12-amd64-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '20.5 kB',
 'tags': [],
 'uuid': '6e4fb67f-6455-42d9-b54e-a58190b4cf5a',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-16uny_w2/slate.git e3d77f11e042cbf6281455ed324a0e78986b37bb

# javascript
31 rules, avg.len. 11.2
## train
PPCR: 0.903348
### report
macro
{'f1-score': 0.46306421937366243,
 'precision': 0.46522327381340506,
 'recall': 0.4626958101615129,
 'support': 51938}
micro
{'f1-score': 0.9489968808964535,
 'precision': 0.9489968808964535,
 'recall': 0.9489968808964535,
 'support': 51938}
weighted
{'f1-score': 0.9444786239456289,
 'precision': 0.9414696775932321,
 'recall': 0.9489968808964535,
 'support': 51938}
### report_full
macro
{'f1-score': 0.4326977456054921,
 'precision': 0.46522327381340506,
 'recall': 0.4114793256723446,
 'support': 57495}
micro
{'f1-score': 0.9008068864053804,
 'precision': 0.9489968808964535,
 'recall': 0.8572745456126619,
 'support': 57495}
weighted
{'f1-score': 0.8793354035626159,
 'precision': 0.9082305606342661,
 'recall': 0.8572745456126619,
 'support': 57495}
## test
PPCR: 0.748552
### report
macro
{'f1-score': 0.2722677053646299,
 'precision': 0.2582765319210638,
 'recall': 0.3058997365627913,
 'support': 646}
micro
{'f1-score': 0.8699690402476781,
 'precision': 0.8699690402476781,
 'recall': 0.8699690402476781,
 'support': 646}
weighted
{'f1-score': 0.8558612492769141,
 'precision': 0.8471627032532625,
 'recall': 0.8699690402476781,
 'support': 646}
### report_full
macro
{'f1-score': 0.2255567224658023,
 'precision': 0.2582765319210638,
 'recall': 0.2091435242156127,
 'support': 863}
micro
{'f1-score': 0.7448641484426775,
 'precision': 0.8699690402476781,
 'recall': 0.6512166859791425,
 'support': 863}
weighted
{'f1-score': 0.7101247413445945,
 'precision': 0.7972053147516192,
 'recall': 0.6512166859791425,
 'support': 863}
```

## javascript
### Summary
21 rules, avg.len. 9.7

| | |
|-|-|
|Min support|110|
|Max support|11795|
|Min confidence|0.9265734553337097|
|Max confidence|0.9996296167373657|

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
| 1 | `  -1.reserved not in {[}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 7017.` |
| 2 | `  +2.roles in {ARGUMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 2540.` |
| 3 | `  +1.reserved = ;<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 2213.` |
| 4 | `  -1.internal_type = CommentLine<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.990. Support: 1210.` |
| 5 | `  -1.internal_type not in {CommentLine}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1350.` |
| 6 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = {<br>	∧ -4.label in {<space>}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.929. Support: 1027.` |
| 7 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.972. Support: 231.` |
| 8 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1112.` |
| 9 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.965. Support: 784.` |
| 10 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.954. Support: 679.` |
| 11 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {MAP}<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 416.` |
| 12 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.reserved not in {,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.997. Support: 164.` |
| 13 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = !<br>	∧ -1.roles not in {MAP}<br>	∧ +1.reserved not in {}}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 249.` |
| 14 | `  -1.internal_type = Identifier<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, ;, {}<br>	∧ +1.reserved = (<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 625.` |
| 15 | `  -1.internal_type not in {CommentLine, Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, {}<br>	∧ -1.roles not in {MAP}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.reserved not in {)}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 1067.` |
| 16 | `  -1.diff_col ≥ 28<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, ;, {}<br>	∧ -1.roles in {COMMENT}<br>	∧ +1.reserved not in {(, }}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 110.` |
| 17 | `  -1.diff_col ≤ 27<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = )<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 323.` |
| 18 | `  -1.diff_col ≤ 27<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, (, ;, {, }}<br>	∧ -2.reserved = [<br>	∧ +1.reserved not in {(, }}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 112.` |
| 19 | `  -1.diff_col ≤ 27<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, ;, {}<br>	∧ -2.reserved = (<br>	∧ +1.reserved = )<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.927. Support: 143.` |
| 20 | `  -1.diff_col ≤ 27<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, ?, {, }}<br>	∧ -1.roles not in {MAP}<br>	∧ -2.label in {<newline>}<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles in {BLOCK} and not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 389.` |
| 21 | `  -1.diff_col ≤ 27<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, ?, {, }}<br>	∧ -1.roles not in {MAP}<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {BLOCK, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 11795.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.714285714285714, "max_conf": 0.9996296167373657, "max_support": 11795, "min_conf": 0.9265734553337097, "min_support": 110, "num_rules": 21}}
```
</details>
