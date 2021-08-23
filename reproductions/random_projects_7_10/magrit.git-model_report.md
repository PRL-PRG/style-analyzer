# Model report for file:///tmp/top-repos-quality-repos-mcjhqiwu/magrit.git HEAD 59cc52f1219053e7d3d3eab506ead231649145a6

### Dump

```json
{'created_at': '2021-08-21 05:41:30',
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
 'size': '21.0 kB',
 'tags': [],
 'uuid': 'a5e962e6-ef8a-4056-a0bc-294ad4cfffd5',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-mcjhqiwu/magrit.git 59cc52f1219053e7d3d3eab506ead231649145a6

# javascript
57 rules, avg.len. 9.8
## train
PPCR: 0.922908
### report
macro
{'f1-score': 0.5809626193125993,
 'precision': 0.5831052736988084,
 'recall': 0.5805407432911878,
 'support': 148075}
micro
{'f1-score': 0.9756744892790815,
 'precision': 0.9756744892790815,
 'recall': 0.9756744892790815,
 'support': 148075}
weighted
{'f1-score': 0.9736961893652372,
 'precision': 0.9722177317840528,
 'recall': 0.9756744892790815,
 'support': 148075}
### report_full
macro
{'f1-score': 0.4822424282952109,
 'precision': 0.5831052736988084,
 'recall': 0.4398858170212558,
 'support': 160444}
micro
{'f1-score': 0.9365582022501046,
 'precision': 0.9756744892790815,
 'recall': 0.9004574804916358,
 'support': 160444}
weighted
{'f1-score': 0.9224976647230498,
 'precision': 0.964396741987053,
 'recall': 0.9004574804916358,
 'support': 160444}
## test
PPCR: 0.925553
### report
macro
{'f1-score': 0.5744950987649194,
 'precision': 0.5844634897807612,
 'recall': 0.5666705295086002,
 'support': 32287}
micro
{'f1-score': 0.9686561154644284,
 'precision': 0.9686561154644284,
 'recall': 0.9686561154644284,
 'support': 32287}
weighted
{'f1-score': 0.9670923188592618,
 'precision': 0.9665206885164994,
 'recall': 0.9686561154644284,
 'support': 32287}
### report_full
macro
{'f1-score': 0.4808935235978247,
 'precision': 0.5844634897807612,
 'recall': 0.43501003022702034,
 'support': 34884}
micro
{'f1-score': 0.931205430915127,
 'precision': 0.9686561154644284,
 'recall': 0.8965428276573787,
 'support': 34884}
weighted
{'f1-score': 0.9183800104871465,
 'precision': 0.9600080815854134,
 'recall': 0.8965428276573787,
 'support': 34884}
```

## javascript
### Summary
33 rules, avg.len. 8.8

| | |
|-|-|
|Min support|101|
|Max support|27934|
|Min confidence|0.9282828569412231|
|Max confidence|0.9998879432678223|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.994. Support: 8405.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1691.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ +4.reserved not in {,}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 1013.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ -4.reserved not in {.}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.991. Support: 8331.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.reserved = }<br>	∧ -5.reserved not in {(}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.reserved = (<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.928. Support: 495.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 619.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 150.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 27934.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 3298.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 846.` |
| 11 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 709.` |
| 12 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +5.reserved = )<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 263.` |
| 13 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +5.reserved not in {)}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>	∧ ^2.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 210.` |
| 14 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 604.` |
| 15 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = else<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 358.` |
| 16 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 119.` |
| 17 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved = (<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 200.` |
| 18 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 8760.` |
| 19 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 649.` |
| 20 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {EXPRESSION, IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 101.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 4323.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 724.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression, ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 19023.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 151.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 4461.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 3190.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 1739.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1223.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.label in {'}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.981. Support: 130.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {)}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, =}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = ArrowFunctionExpression<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 179.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =, [, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {)}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {{}<br>	∧ ^1.internal_type = ArrowFunctionExpression<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 429.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, [, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {)}<br>	∧ -3.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {{}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 131.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, [, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {)}<br>	∧ -3.reserved not in {), >}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ;, ], }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 5152.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.848484848484848, "max_conf": 0.9998879432678223, "max_support": 27934, "min_conf": 0.9282828569412231, "min_support": 101, "num_rules": 33}}
```
</details>
